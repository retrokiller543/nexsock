use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use futures::StreamExt;
use nexsock_client::Client;
use nexsock_protocol::commands::{CommandPayload, list_services::ServiceInfo};
use ratatui::DefaultTerminal;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use super::events::TuiEvent;
use super::ui::{render_ui, AppState, ViewMode};

/// Main TUI application
pub struct TuiApp {
    client: Client,
    refresh_rate: Duration,
    initial_service: Option<String>,
}

impl TuiApp {
    /// Create a new TUI application
    pub fn new(
        client: Client,
        refresh_rate: Duration,
        initial_service: Option<String>,
    ) -> Self {
        Self {
            client,
            refresh_rate,
            initial_service,
        }
    }

    /// Run the TUI application with full async event handling
    pub async fn run(self, mut terminal: DefaultTerminal) -> Result<()> {
        // Initialize app state
        let mut app_state = AppState::new(self.initial_service.clone());
        
        // Load initial services list
        self.load_services(&mut app_state).await?;

        // Setup event channels
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let mut event_stream = UnboundedReceiverStream::new(event_rx);

        // Start background tasks
        self.spawn_background_tasks(event_tx.clone()).await?;

        // Main async event loop using tokio::select!
        loop {
            tokio::select! {
                // Handle user input events
                event = self.read_terminal_event() => {
                    match event? {
                        Event::Key(key) => {
                            if self.handle_key_event(key, &mut app_state, &event_tx).await? {
                                break; // Exit requested
                            }
                        }
                        Event::Mouse(mouse) => {
                            event_tx.send(TuiEvent::Mouse(mouse))?;
                        }
                        _ => {}
                    }
                }

                // Handle internal events (service updates, logs, etc.)
                Some(tui_event) = event_stream.next() => {
                    if self.handle_tui_event(tui_event, &mut app_state).await? {
                        break; // Exit requested
                    }
                }

                // Periodic refresh tick
                _ = tokio::time::sleep(self.refresh_rate) => {
                    event_tx.send(TuiEvent::Tick)?;
                }
            }

            // Render UI
            terminal.draw(|frame| render_ui(frame, &app_state))?;
        }

        Ok(())
    }

    /// Load services from daemon
    async fn load_services(&self, app_state: &mut AppState) -> Result<()> {
        let response = self.client.list_services().await?;
        
        if let CommandPayload::ListServices(list) = response {
            app_state.update_services(list.services);
        }
        
        Ok(())
    }

    /// Read terminal events asynchronously
    async fn read_terminal_event(&self) -> Result<Event> {
        let mut event_stream = EventStream::new();
        
        match event_stream.next().await {
            Some(Ok(event)) => Ok(event),
            Some(Err(e)) => Err(e.into()),
            None => Err(anyhow::anyhow!("Event stream ended")),
        }
    }

    /// Handle keyboard input
    async fn handle_key_event(
        &self,
        key: KeyEvent,
        app_state: &mut AppState,
        event_tx: &mpsc::UnboundedSender<TuiEvent>,
    ) -> Result<bool> {
        // Global key bindings
        match (key.code, key.modifiers) {
            // Quit with Ctrl+C or 'q'
            (KeyCode::Char('c'), KeyModifiers::CONTROL) |
            (KeyCode::Char('q'), KeyModifiers::NONE) => {
                return Ok(true);
            }
            
            // Navigation
            (KeyCode::Up, _) | (KeyCode::Char('k'), _) => {
                app_state.select_previous();
            }
            (KeyCode::Down, _) | (KeyCode::Char('j'), _) => {
                app_state.select_next();
            }
            
            // View modes
            (KeyCode::Char('1'), _) => {
                app_state.set_view_mode(ViewMode::ServicesList);
            }
            (KeyCode::Char('2'), _) => {
                app_state.set_view_mode(ViewMode::ServiceDetails);
            }
            (KeyCode::Char('3'), _) => {
                app_state.set_view_mode(ViewMode::Logs);
            }
            (KeyCode::Char('4'), _) => {
                app_state.set_view_mode(ViewMode::AddService);
            }
            
            // Service actions
            (KeyCode::Char('s'), _) => {
                if let Some(service) = app_state.selected_service() {
                    self.start_service(&service.name, event_tx).await?;
                }
            }
            (KeyCode::Char('S'), _) => {
                if let Some(service) = app_state.selected_service() {
                    self.stop_service(&service.name, event_tx).await?;
                }
            }
            (KeyCode::Char('r'), _) => {
                if let Some(service) = app_state.selected_service() {
                    self.restart_service(&service.name, event_tx).await?;
                }
            }
            
            // Refresh
            (KeyCode::F5, _) | (KeyCode::Char('R'), _) => {
                self.refresh_services(event_tx).await?;
            }
            
            _ => {}
        }
        
        Ok(false)
    }

    /// Handle internal TUI events
    async fn handle_tui_event(
        &self,
        event: TuiEvent,
        app_state: &mut AppState,
    ) -> Result<bool> {
        match event {
            TuiEvent::Key(_) | TuiEvent::Mouse(_) => {
                // These are handled elsewhere
            }
            
            TuiEvent::ServiceUpdate(payload) => {
                if let CommandPayload::ListServices(list) = payload {
                    app_state.update_services(list.services);
                }
            }
            
            TuiEvent::LogEntry { service_name, timestamp, message } => {
                app_state.add_log_entry(service_name, timestamp, message);
            }
            
            TuiEvent::TaskComplete { task_id: _, result } => {
                match result {
                    Ok(payload) => {
                        app_state.set_status("Task completed successfully".to_string());
                        if let CommandPayload::ListServices(list) = payload {
                            app_state.update_services(list.services);
                        }
                    }
                    Err(error) => {
                        app_state.set_error(format!("Task failed: {}", error));
                    }
                }
            }
            
            TuiEvent::Tick => {
                app_state.clear_temporary_messages();
            }
            
            TuiEvent::Quit => {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Start a service asynchronously
    async fn start_service(
        &self,
        service_name: &str,
        event_tx: &mpsc::UnboundedSender<TuiEvent>,
    ) -> Result<()> {
        let client = self.client.clone();
        let service_name = service_name.to_string();
        let event_tx = event_tx.clone();
        
        tokio::spawn(async move {
            let task_id = format!("start_{}", service_name);
            
            let result = client.start_service(&service_name, vec![]).await
                .map_err(|e| e.to_string());
            
            let _ = event_tx.send(TuiEvent::TaskComplete { task_id, result });
        });
        
        Ok(())
    }

    /// Stop a service asynchronously
    async fn stop_service(
        &self,
        service_name: &str,
        event_tx: &mpsc::UnboundedSender<TuiEvent>,
    ) -> Result<()> {
        let client = self.client.clone();
        let service_name = service_name.to_string();
        let event_tx = event_tx.clone();
        
        tokio::spawn(async move {
            let task_id = format!("stop_{}", service_name);
            
            let result = client.stop_service(&service_name).await
                .map_err(|e| e.to_string());
            
            let _ = event_tx.send(TuiEvent::TaskComplete { task_id, result });
        });
        
        Ok(())
    }

    /// Restart a service asynchronously
    async fn restart_service(
        &self,
        service_name: &str,
        event_tx: &mpsc::UnboundedSender<TuiEvent>,
    ) -> Result<()> {
        let client = self.client.clone();
        let service_name = service_name.to_string();
        let event_tx = event_tx.clone();
        
        tokio::spawn(async move {
            let task_id = format!("restart_{}", service_name);
            
            // Stop then start
            let stop_result = client.stop_service(&service_name).await;
            if stop_result.is_ok() {
                tokio::time::sleep(Duration::from_millis(500)).await;
                let start_result = client.start_service(&service_name, vec![]).await;
                let result = start_result.map_err(|e| e.to_string());
                let _ = event_tx.send(TuiEvent::TaskComplete { task_id, result });
            } else {
                let result = stop_result.map_err(|e| e.to_string());
                let _ = event_tx.send(TuiEvent::TaskComplete { task_id, result });
            }
        });
        
        Ok(())
    }

    /// Refresh services list asynchronously
    async fn refresh_services(
        &self,
        event_tx: &mpsc::UnboundedSender<TuiEvent>,
    ) -> Result<()> {
        let client = self.client.clone();
        let event_tx = event_tx.clone();
        
        tokio::spawn(async move {
            let task_id = "refresh_services".to_string();
            
            let result = client.list_services().await
                .map_err(|e| e.to_string());
            
            let _ = event_tx.send(TuiEvent::TaskComplete { task_id, result });
        });
        
        Ok(())
    }

    /// Spawn background tasks for real-time updates
    async fn spawn_background_tasks(
        &self,
        event_tx: mpsc::UnboundedSender<TuiEvent>,
    ) -> Result<()> {
        // Background service status polling
        let client = self.client.clone();
        let tx = event_tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            
            loop {
                interval.tick().await;
                
                if let Ok(response) = client.list_services().await {
                    let _ = tx.send(TuiEvent::ServiceUpdate(response));
                }
            }
        });

        // TODO: Add log streaming background task
        // TODO: Add health monitoring background task
        
        Ok(())
    }
}