mod components;
mod daemon_client;
mod embedded;
mod endpoints;
mod error;
mod services;
mod state;
pub(crate) mod templates;
mod traits;

use crate::endpoints::api::service::get::get_services;
use crate::endpoints::fallback::static_handler;
use anyhow::Context;
use axum::handler::Handler;
use axum::http::{Request, Response};
use axum::routing::{delete, post};
use axum::{routing::get, Router};
use axum_response_cache::CacheLayer;
use endpoints::get_services::get_nexsock_service;
use endpoints::index;
use state::AppState;
use std::time::Duration;
use tokio::net::{TcpListener, ToSocketAddrs};
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, Span};

type Result<T, E = error::ServiceError> = std::result::Result<T, E>;

#[inline]
#[tracing::instrument]
pub async fn app() -> anyhow::Result<Router> {
    let state = AppState::new().await?;
    let compression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);
    let cache = CacheLayer::with_lifespan(60).add_response_headers();

    Ok(Router::new()
        .route("/", get(index::index_html))
        .route("/services", get(get_services))
        .route("/services/{id}", get(get_nexsock_service))
        .route(
            "/api/services/{service_id}",
            delete(endpoints::api::service::delete::remove_service),
        )
        .route(
            "/services",
            post(endpoints::api::service::add::add_service_endpoint),
        )
        .route(
            "/services/{service_id}/start",
            post(endpoints::api::service::start::start_service),
        )
        .route(
            "/services/{service_id}/stop",
            post(endpoints::api::service::stop::stop_service),
        )
        .route(
            "/api/templates/env-var-pair",
            get(endpoints::templates::env_var_pair),
        )
        .route(
            "/api/templates/config-section",
            get(endpoints::templates::config_section),
        )
        .route(
            "/api/templates/config-modal",
            get(endpoints::templates::config_modal),
        )
        .route(
            "/api/templates/config-modal-content",
            get(endpoints::templates::config_modal_content),
        )
        .route(
            "/api/templates/git-section",
            get(endpoints::templates::git_section),
        )
        .route(
            "/api/templates/git-status",
            get(endpoints::templates::git_status_template),
        )
        .route(
            "/api/templates/git-modal",
            get(endpoints::templates::git_modal),
        )
        .route(
            "/api/templates/git-branches",
            get(endpoints::templates::git_branches),
        )
        .route("/api/templates/git-log", get(endpoints::templates::git_log))
        // Git endpoints
        .route(
            "/api/services/{service_id}/git/status",
            get(endpoints::api::service::git::git_status),
        )
        .route(
            "/api/services/{service_id}/git/branches",
            get(endpoints::api::service::git::git_branches),
        )
        .route(
            "/api/services/{service_id}/git/log",
            get(endpoints::api::service::git::git_log),
        )
        .route(
            "/api/services/{service_id}/git/checkout/branch",
            post(endpoints::api::service::git::git_checkout_branch),
        )
        .route(
            "/api/services/{service_id}/git/checkout/commit",
            post(endpoints::api::service::git::git_checkout_commit),
        )
        .route(
            "/api/services/{service_id}/git/pull",
            post(endpoints::api::service::git::git_pull),
        )
        .fallback(static_handler.layer(cache))
        .layer(compression_layer)
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
                        "finished {} {:?} with {} headers in {:?}",
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
