use nexsock_config::PROJECT_DIRECTORIES;
use nexsock_web::serve_default;
use tosic_utils::logging::init_tracing_layered;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
/// Initializes logging and starts the default web server asynchronously.
///
/// Sets up layered tracing-based logging to a file in the application's data directory,
/// then launches the default web server. Propagates any errors encountered during setup or server startup.
///
/// # Returns
///
/// An `anyhow::Result` indicating success or containing any error that occurred.
///
/// # Examples
///
/// ```no_run
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     main().await
/// }
/// ```
async fn main() -> anyhow::Result<()> {
    let logging_path = PROJECT_DIRECTORIES.data_dir().join("logs");

    let _guard = init_tracing_layered(Some((logging_path, "nexsock-web.log")))?;

    serve_default().await
}
