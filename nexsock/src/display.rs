use std::io::{self, Write};

use nexsock_protocol::commands::CommandPayload;

pub mod formatters;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
    Plain,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Table
    }
}

#[derive(Debug, Clone)]
pub struct DisplayOptions {
    pub format: OutputFormat,
    pub verbose: bool,
    pub no_headers: bool,
    pub color: bool,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            format: OutputFormat::default(),
            verbose: false,
            no_headers: false,
            color: atty::is(atty::Stream::Stdout),
        }
    }
}

pub trait CliDisplay {
    fn display(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> 
    where 
        Self: serde::Serialize,
    {
        match options.format {
            OutputFormat::Table => self.display_table(options, writer),
            OutputFormat::Json => self.display_json(writer),
            OutputFormat::Yaml => self.display_yaml(writer),
            OutputFormat::Plain => self.display_plain(options, writer),
        }
    }
    
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()>;
    fn display_plain(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()>;
    
    fn display_json(&self, writer: &mut dyn Write) -> io::Result<()>
    where
        Self: serde::Serialize,
    {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        writeln!(writer, "{}", json)
    }
    
    fn display_yaml(&self, writer: &mut dyn Write) -> io::Result<()>
    where
        Self: serde::Serialize,
    {
        let yaml = serde_yaml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        write!(writer, "{}", yaml)
    }
}

impl CliDisplay for CommandPayload {
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        match self {
            CommandPayload::Status(status) => status.display_table(options, writer),
            CommandPayload::ListServices(list) => list.display_table(options, writer),
            CommandPayload::ServiceConfig(config) => config.display_table(options, writer),
            CommandPayload::Dependencies(deps) => deps.display_table(options, writer),
            CommandPayload::GitLog(log) => log.display_table(options, writer),
            CommandPayload::GitBranches(branches) => branches.display_table(options, writer),
            CommandPayload::GitStatus(status) => status.display_table(options, writer),
            CommandPayload::Stdout(output) => write!(writer, "{}", output),
            CommandPayload::Error(error) => error.display_table(options, writer),
            CommandPayload::Empty => Ok(()),
            _ => writeln!(writer, "Unknown response type"),
        }
    }

    fn display_plain(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        match self {
            CommandPayload::Status(status) => status.display_plain(options, writer),
            CommandPayload::ListServices(list) => list.display_plain(options, writer),
            CommandPayload::ServiceConfig(config) => config.display_plain(options, writer),
            CommandPayload::Dependencies(deps) => deps.display_plain(options, writer),
            CommandPayload::GitLog(log) => log.display_plain(options, writer),
            CommandPayload::GitBranches(branches) => branches.display_plain(options, writer),
            CommandPayload::GitStatus(status) => status.display_plain(options, writer),
            CommandPayload::Stdout(output) => write!(writer, "{}", output),
            CommandPayload::Error(error) => error.display_plain(options, writer),
            CommandPayload::Empty => Ok(()),
            _ => writeln!(writer, "Unknown response"),
        }
    }
}

pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    
    pub const BRIGHT_RED: &str = "\x1b[91m";
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    pub const BRIGHT_BLUE: &str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
}

pub mod utils {
    use std::io::{self, Write};
    use super::{colors, DisplayOptions};

    pub fn colorize(text: &str, color: &str, options: &DisplayOptions) -> String {
        if options.color {
            format!("{}{}{}", color, text, colors::RESET)
        } else {
            text.to_string()
        }
    }

    pub fn status_color(status: &str) -> &'static str {
        match status.to_lowercase().as_str() {
            "running" => colors::BRIGHT_GREEN,
            "starting" => colors::YELLOW,
            "stopping" => colors::YELLOW,
            "stopped" => colors::DIM,
            "failed" => colors::BRIGHT_RED,
            _ => colors::WHITE,
        }
    }

    pub fn write_table_row(
        writer: &mut dyn Write,
        columns: &[&str],
        widths: &[usize],
        options: &DisplayOptions,
    ) -> io::Result<()> {
        for (i, (column, &width)) in columns.iter().zip(widths.iter()).enumerate() {
            if i > 0 {
                write!(writer, "  ")?;
            }
            write!(writer, "{:<width$}", column, width = width)?;
        }
        writeln!(writer)
    }

    pub fn write_table_separator(
        writer: &mut dyn Write,
        widths: &[usize],
        options: &DisplayOptions,
    ) -> io::Result<()> {
        if !options.color {
            return Ok(());
        }
        
        for (i, &width) in widths.iter().enumerate() {
            if i > 0 {
                write!(writer, "  ")?;
            }
            write!(writer, "{}", "─".repeat(width))?;
        }
        writeln!(writer)
    }

    pub fn truncate_string(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else if max_len <= 3 {
            "...".to_string()
        } else {
            format!("{}...", &s[..max_len - 3])
        }
    }

    pub fn format_timestamp(timestamp: &str) -> String {
        timestamp.split('T')
            .next()
            .unwrap_or(timestamp)
            .to_string()
    }

    pub fn format_commit_hash(hash: &str, short: bool) -> &str {
        if short && hash.len() > 7 {
            &hash[..7]
        } else {
            hash
        }
    }
}