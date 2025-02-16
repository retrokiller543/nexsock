#[tokio::main(flavor = "current_thread")]
async fn main() -> nexsockd::prelude::Result<()> {
    nexsockd::run_daemon().await
}
