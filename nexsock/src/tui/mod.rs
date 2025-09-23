mod app;
mod events;
mod ui;

pub use app::TuiApp;
pub use events::TuiEvent;

use anyhow::Result;
use nexsock_client::Client;
use std::time::Duration;

/// Launch the TUI application
pub async fn run_tui(
    client: Client,
    refresh_rate: Duration,
    initial_service: Option<String>,
) -> Result<()> {
    // Setup terminal
    let terminal = ratatui::init();
    
    // Create and run app
    let result = TuiApp::new(client, refresh_rate, initial_service)?
        .run(terminal)
        .await;
    
    // Cleanup
    ratatui::restore();
    result
}