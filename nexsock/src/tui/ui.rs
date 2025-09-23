use nexsock_protocol::commands::list_services::ServiceInfo;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState, Tabs},
    Frame,
};
use std::collections::HashMap;

/// Different view modes for the TUI
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    ServicesList,
    ServiceDetails,
    Logs,
    AddService,
}

/// Main application state
pub struct AppState {
    pub services: Vec<ServiceInfo>,
    pub selected_service_index: Option<usize>,
    pub view_mode: ViewMode,
    pub status_message: Option<String>,
    pub error_message: Option<String>,
    pub logs: HashMap<String, Vec<String>>,
}

impl AppState {
    pub fn new(initial_service: Option<String>) -> Self {
        let mut state = Self {
            services: Vec::new(),
            selected_service_index: None,
            view_mode: ViewMode::ServicesList,
            status_message: None,
            error_message: None,
            logs: HashMap::new(),
        };

        // Select initial service if provided
        if let Some(service_name) = initial_service {
            // Will be set when services are loaded
            state.status_message = Some(format!("Looking for service: {}", service_name));
        }

        state
    }

    pub fn update_services(&mut self, new_services: Vec<ServiceInfo>) {
        // Preserve selection if possible
        let selected_name = self.selected_service().map(|s| s.name.clone());
        
        self.services = new_services;
        
        // Restore selection
        if let Some(name) = selected_name {
            self.selected_service_index = self.services
                .iter()
                .position(|s| s.name == name);
        }
        
        // Ensure selection is valid
        if self.selected_service_index.is_some() && 
           self.selected_service_index >= Some(self.services.len()) {
            self.selected_service_index = self.services.len().checked_sub(1);
        }
        
        // Set initial selection if none exists
        if self.selected_service_index.is_none() && !self.services.is_empty() {
            self.selected_service_index = Some(0);
        }
        
    }

    pub fn select_previous(&mut self) {
        if let Some(index) = self.selected_service_index {
            if index > 0 {
                self.selected_service_index = Some(index - 1);
            } else if !self.services.is_empty() {
                self.selected_service_index = Some(self.services.len() - 1);
            }
        } else if !self.services.is_empty() {
            self.selected_service_index = Some(self.services.len() - 1);
        }
    }

    pub fn select_next(&mut self) {
        if let Some(index) = self.selected_service_index {
            if index < self.services.len() - 1 {
                self.selected_service_index = Some(index + 1);
            } else {
                self.selected_service_index = Some(0);
            }
        } else if !self.services.is_empty() {
            self.selected_service_index = Some(0);
        }
    }

    pub fn selected_service(&self) -> Option<&ServiceInfo> {
        self.selected_service_index
            .and_then(|index| self.services.get(index))
    }

    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
    }

    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
        self.error_message = None;
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.status_message = None;
    }

    pub fn add_log_entry(&mut self, service_name: String, timestamp: String, message: String) {
        let log_line = format!("[{}] {}", timestamp, message);
        self.logs.entry(service_name.clone()).or_insert_with(Vec::new).push(log_line);
        
        // Keep only last 1000 lines
        let logs = self.logs.get_mut(&service_name).unwrap();
        if logs.len() > 1000 {
            logs.drain(0..logs.len() - 1000);
        }
    }

    pub fn clear_temporary_messages(&mut self) {
        // Clear status messages after some time
        // This would be enhanced with timestamps in a real implementation
    }
}

/// Main UI rendering function
pub fn render_ui(frame: &mut Frame, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status bar
        ])
        .split(frame.area());

    // Render tabs
    render_tabs(frame, chunks[0], app_state.view_mode);

    // Render main content based on view mode
    match app_state.view_mode {
        ViewMode::ServicesList => render_services_list(frame, chunks[1], app_state),
        ViewMode::ServiceDetails => render_service_details(frame, chunks[1], app_state),
        ViewMode::Logs => render_logs(frame, chunks[1], app_state),
        ViewMode::AddService => render_add_service(frame, chunks[1], app_state),
    }

    // Render status bar
    render_status_bar(frame, chunks[2], app_state);
}

fn render_tabs(frame: &mut Frame, area: Rect, current_mode: ViewMode) {
    let titles = vec!["Services (1)", "Details (2)", "Logs (3)", "Add Service (4)"];
    
    let selected_index = match current_mode {
        ViewMode::ServicesList => 0,
        ViewMode::ServiceDetails => 1,
        ViewMode::Logs => 2,
        ViewMode::AddService => 3,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("nexsock TUI"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .select(selected_index);

    frame.render_widget(tabs, area);
}

fn render_services_list(frame: &mut Frame, area: Rect, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left side: Services table
    let headers = ["ID", "Name", "Status", "Port"];
    let rows: Vec<Row> = app_state.services.iter().map(|service| {
        let status_style = match service.state.to_string().as_str() {
            "Running" => Style::default().fg(Color::Green),
            "Stopped" => Style::default().fg(Color::Red),
            "Starting" => Style::default().fg(Color::Yellow),
            "Failed" => Style::default().fg(Color::Magenta),
            _ => Style::default().fg(Color::Gray),
        };

        Row::new(vec![
            service.id.to_string(),
            service.name.clone(),
            service.state.to_string(),
            service.port.to_string(),
        ]).style(status_style)
    }).collect();

    let table = Table::new(rows, [
        Constraint::Length(4),
        Constraint::Min(20),
        Constraint::Length(10),
        Constraint::Length(6),
    ])
    .header(
        Row::new(headers)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .bottom_margin(1),
    )
    .block(Block::default().borders(Borders::ALL).title(
        if let Some(service) = app_state.selected_service() {
            format!("Services - Selected: {}", service.name)
        } else {
            "Services - No selection".to_string()
        }
    ))
    .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
    .highlight_symbol("► ");

    // Create a local table state based on current selection
    let mut table_state = TableState::default();
    table_state.select(app_state.selected_service_index);
    
    frame.render_stateful_widget(table, chunks[0], &mut table_state);

    // Right side: Key bindings help
    let help_text = vec![
        Line::from(vec![
            Span::styled("Controls:", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation:", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("↑/k", Style::default().fg(Color::Yellow)),
            Span::raw("  Move selection up"),
        ]),
        Line::from(vec![
            Span::styled("↓/j", Style::default().fg(Color::Yellow)),
            Span::raw("  Move selection down"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Service Actions:", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("s", Style::default().fg(Color::Green)),
            Span::raw("    Start selected service"),
        ]),
        Line::from(vec![
            Span::styled("S", Style::default().fg(Color::Red)),
            Span::raw("    Stop selected service"),
        ]),
        Line::from(vec![
            Span::styled("r", Style::default().fg(Color::Blue)),
            Span::raw("    Restart selected service"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Other:", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("F5/R", Style::default().fg(Color::Cyan)),
            Span::raw("  Refresh services"),
        ]),
        Line::from(vec![
            Span::styled("q", Style::default().fg(Color::White)),
            Span::raw("    Quit application"),
        ]),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"));

    frame.render_widget(help, chunks[1]);
}

fn render_service_details(frame: &mut Frame, area: Rect, app_state: &AppState) {
    if let Some(service) = app_state.selected_service() {
        let details = vec![
            Line::from(vec![
                Span::styled("ID: ", Style::default().fg(Color::Yellow)),
                Span::raw(service.id.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Name: ", Style::default().fg(Color::Yellow)),
                Span::raw(&service.name),
            ]),
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    service.state.to_string(),
                    match service.state.to_string().as_str() {
                        "Running" => Style::default().fg(Color::Green),
                        "Stopped" => Style::default().fg(Color::Red),
                        _ => Style::default().fg(Color::Gray),
                    }
                ),
            ]),
            Line::from(vec![
                Span::styled("Port: ", Style::default().fg(Color::Yellow)),
                Span::raw(service.port.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Dependencies: ", Style::default().fg(Color::Yellow)),
                Span::raw(if service.has_dependencies { "Yes" } else { "No" }),
            ]),
        ];

        let paragraph = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title("Service Details"));

        frame.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new("No service selected")
            .block(Block::default().borders(Borders::ALL).title("Service Details"));

        frame.render_widget(paragraph, area);
    }
}

fn render_logs(frame: &mut Frame, area: Rect, app_state: &AppState) {
    if let Some(service) = app_state.selected_service() {
        let logs = app_state.logs.get(&service.name)
            .map(|logs| logs.iter().map(|line| Line::from(line.as_str())).collect())
            .unwrap_or_else(|| vec![Line::from("No logs available")]);

        let paragraph = Paragraph::new(logs)
            .block(Block::default().borders(Borders::ALL).title(format!("Logs - {}", service.name)));

        frame.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new("No service selected")
            .block(Block::default().borders(Borders::ALL).title("Logs"));

        frame.render_widget(paragraph, area);
    }
}

fn render_add_service(frame: &mut Frame, area: Rect, _app_state: &AppState) {
    let paragraph = Paragraph::new("Add Service form coming soon...")
        .block(Block::default().borders(Borders::ALL).title("Add Service"));

    frame.render_widget(paragraph, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, app_state: &AppState) {
    let status_text = if let Some(error) = &app_state.error_message {
        Line::from(vec![
            Span::styled("Error: ", Style::default().fg(Color::Red)),
            Span::raw(error),
        ])
    } else if let Some(status) = &app_state.status_message {
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::Green)),
            Span::raw(status),
        ])
    } else {
        Line::from("Ready")
    };

    let paragraph = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(paragraph, area);
}