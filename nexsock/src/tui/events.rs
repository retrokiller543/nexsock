use crossterm::event::{KeyEvent, MouseEvent};
use nexsock_protocol::commands::CommandPayload;

/// Events that the TUI can handle
#[derive(Debug, Clone)]
pub enum TuiEvent {
    /// User input events
    Key(KeyEvent),
    Mouse(MouseEvent),
    
    /// Real-time updates from the nexsock daemon
    ServiceUpdate(CommandPayload),
    
    /// Log entries from services
    LogEntry {
        service_name: String,
        timestamp: String,
        message: String,
    },
    
    /// Background task completion
    TaskComplete {
        task_id: String,
        result: Result<CommandPayload, String>,
    },
    
    /// Periodic refresh tick
    Tick,
    
    /// Request to quit the application
    Quit,
}