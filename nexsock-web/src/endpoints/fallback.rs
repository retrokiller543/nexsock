use crate::embedded::Public;
use axum::body::Body;
use axum::http::{header, HeaderValue, Response, StatusCode};

pub async fn static_handler(uri: axum::http::Uri) -> Response<Body> {
    let path = uri.path().trim_start_matches('/');

    match Public::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime.as_ref()).unwrap(),
                )
                .header(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=3600, stale-while-revalidate=86400"),
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
