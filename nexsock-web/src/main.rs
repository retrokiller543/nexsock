use anyhow::{anyhow, Context};
use axum::{extract::State, routing::get, Router};
use std::{path::PathBuf, sync::Arc};

use directories::ProjectDirs;
use nexsock_client::Client;
use nexsock_protocol::commands::list_services::ListServicesCommand;
use rust_html::{rhtml, Template};
use tokio::net::TcpListener;
#[cfg(windows)]
use tokio::net::TcpStream;

struct AppState {
    #[cfg(unix)]
    socket_path: PathBuf,
    #[cfg(windows)]
    port_file: PathBuf,
}

#[cfg(windows)]
async fn get_daemon_port(port_file: &PathBuf) -> anyhow::Result<String> {
    let port_str = fs::read_to_string(port_file)?;
    let port = port_str.trim().parse::<u16>()?;
    Ok(port)
}

async fn connect_to_daemon(state: &AppState) -> anyhow::Result<String> {
    let mut client = Client::connect(&state.socket_path).await?;

    let res = client.execute_command(ListServicesCommand::new()).await?;

    if res.is_list_services() {
        let services = res.unwrap_list_services();
        Ok(format!(
            "Daemon is running with these available services to control: {services:#?}"
        ))
    } else {
        Ok("Daemon is running".to_string())
    }
}

async fn serve_html(State(state): State<Arc<AppState>>) -> Template {
    let status = connect_to_daemon(&state)
        .await
        .unwrap_or_else(|e| format!("Error connecting to daemon: {}", e));

    rhtml!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Daemon Status</title>
            <style>
                body {{
                    font-family: system-ui, -apple-system, sans-serif;
                    max-width: 800px;
                    margin: 0 auto;
                    padding: 20px;
                }}
                .status {{
                    padding: 15px;
                    border-radius: 4px;
                    background: #f0f0f0;
                    margin: 20px 0;
                }}
            </style>
        </head>
        <body>
            <h1>Daemon Status</h1>
            <div class="status">
                <p>Status: {status}</p>
            </div>
        </body>
        </html>
    "#
    )
}

fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "your-org", "your-app")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project_dirs =
        get_project_dirs().ok_or(anyhow!("Failed to determine project directories"))?;

    let state = Arc::new(AppState {
        #[cfg(unix)]
        socket_path: PathBuf::from("/tmp/nexsockd.sock"), // Make configurable

        #[cfg(windows)]
        port_file: project_dirs.cache_dir().join("daemon-port"),
    });

    let app = Router::new().route("/", get(serve_html)).with_state(state);

    let addr = TcpListener::bind("127.0.0.1:5050")
        .await
        .context("failed to bind port")?;
    println!("Web interface available at http://{}", addr.local_addr()?);

    axum::serve(addr, app).await?;

    Ok(())
}
