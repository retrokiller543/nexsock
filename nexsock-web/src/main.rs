mod components;
mod endpoints;
mod layout;
mod templates;
mod traits;

use anyhow::{bail, Context};
use axum::body::Body;
use axum::http::{header, HeaderValue, Request, Response, StatusCode};
use axum::routing::post;
use axum::{routing::get, Router};
use components::service_basic::ServiceBasic;
use endpoints::get_services::get_nexsock_service;
use endpoints::index;
use nexsock_client::Client;
use nexsock_config::{NexsockConfig, SocketRef};
use nexsock_protocol::commands::list_services::ListServicesCommand;
use rust_embed::RustEmbed;
use std::sync::Arc;
use std::time::Duration;
#[cfg(windows)]
use tokio::net::TcpStream;
use tokio::net::{TcpListener, ToSocketAddrs};
use tosic_utils::logging::init_tracing;
use tower_http::trace::TraceLayer;
use tracing::{info, Span};

#[derive(Clone, RustEmbed)]
#[folder = "public"]
struct Public;

async fn static_handler(uri: axum::http::Uri) -> Response<Body> {
    let path = uri.path().trim_start_matches('/');

    match Public::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime.as_ref()).unwrap(),
                )
                .body(Body::from(content.data.to_vec()))
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("404 Not Found"))
            .unwrap(),
    }
}

struct AppState {
    config: NexsockConfig,
}

impl AppState {
    async fn new() -> anyhow::Result<Self> {
        let config = NexsockConfig::new()?;

        Ok(Self { config })
    }
}

#[allow(unused_variables)]
async fn connect_to_client(socket_ref: &SocketRef) -> anyhow::Result<Client> {
    let client = match socket_ref {
        SocketRef::Port(port) => {
            #[cfg(unix)]
            bail!("When on Unix Tcp sockets are not available, please modify config to be a path to the socket file");
            #[cfg(windows)]
            Client::connect(format!("127.0.0.1:{port}")).await?
        }
        SocketRef::Path(path) => {
            #[cfg(windows)]
            bail!("Unix sockets are not available, please modify config to be a path to the port where the daemon is running");
            #[cfg(unix)]
            Client::connect(path).await?
        }
    };

    Ok(client)
}

async fn list_services(state: &AppState) -> anyhow::Result<Vec<ServiceBasic>> {
    let mut client = connect_to_client(state.config.socket()).await?;
    let res = client.execute_command(ListServicesCommand::new()).await?;

    if res.is_list_services() {
        let services = res.unwrap_list_services();

        Ok(ServiceBasic::from_iter(services.services))
    } else {
        Ok(Vec::new())
    }
}

#[inline]
#[tracing::instrument]
pub async fn app() -> anyhow::Result<Router> {
    let state = Arc::new(AppState::new().await?);

    Ok(Router::new()
        .route("/", get(index::index_html))
        .route("/service/{id}", get(get_nexsock_service))
        .route(
            "/api/service/{service_id}/start",
            post(crate::endpoints::api::start_service),
        )
        .route(
            "/api/service/{service_id}/stop",
            post(crate::endpoints::api::stop_service),
        )
        .fallback(static_handler)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_request(|request: &Request<_>, _span: &Span| {
                    info!("started {} {}", request.method(), request.uri());
                })
                .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                    info!(
                        "finished {} {:?} with {} header in {:?}",
                        response.status(),
                        response.version(),
                        response.headers().len(),
                        latency,
                    );
                }),
        )
        .with_state(state))
}

#[inline]
#[tracing::instrument(skip_all)]
pub async fn serve(app: Router, socket_addr: impl ToSocketAddrs) -> anyhow::Result<()> {
    let socket = TcpListener::bind(socket_addr)
        .await
        .context("Failed to bind port")?;

    info!("Listening on http://{}", socket.local_addr()?);

    axum::serve(socket, app)
        .await
        .context("Failed to serve axum server")
}

#[inline]
#[tracing::instrument]
pub async fn serve_default() -> anyhow::Result<()> {
    let app = app().await.context("Failed to construct the App")?;

    serve(app, "0.0.0.0:5050").await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_tracing("nexsock-web.log")?;

    serve_default().await
}
