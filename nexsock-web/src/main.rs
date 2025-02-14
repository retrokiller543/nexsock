use nexsock_config::PROJECT_DIRECTORIES;
use nexsock_web::serve_default;
use tosic_utils::logging::init_tracing_layered;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let logging_path = PROJECT_DIRECTORIES.data_dir().join("logs");

    let _guard = init_tracing_layered(Some((logging_path, "nexsock-web.log")))?;

    serve_default().await
}
