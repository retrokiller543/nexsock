use nexsock_web::serve_default;
use tosic_utils::logging::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_tracing("nexsock-web.log")?;

    serve_default().await
}
