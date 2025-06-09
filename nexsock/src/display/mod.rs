#![doc = include_str!("README.md")]

use std::io::{self, Write};

use nexsock_protocol::commands::CommandPayload;

pub mod simple_formatter;
pub mod protocol_formatters;

pub use simple_formatter::{CollectionFormattable, CompactTableFormattable, SimpleFormattable, SimpleFormatter};

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
    pub detailed: bool,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            format: OutputFormat::default(),
            verbose: false,
            no_headers: false,
            color: atty::is(atty::Stream::Stdout),
            detailed: false,
        }
    }
}

/// Simplified CliDisplay trait that uses the new formatter system
pub trait CliDisplay {
    fn display(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()>;
}

/// Helper function to display any formattable data with the given options
pub fn display_data<T>(
    data: &T,
    options: &DisplayOptions,
    writer: &mut dyn Write,
) -> io::Result<()>
where
    T: SimpleFormattable + serde::Serialize,
{
    let formatter = SimpleFormatter::from_options(options);
    formatter.format_simple(data, writer)
}

// We'll implement Formattable for the protocol types in a separate file
// For now, let's keep the old CommandPayload implementation but use display_data
impl CliDisplay for CommandPayload {
    fn display(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        let formatter = SimpleFormatter::from_options(options);
        
        match self {
            CommandPayload::Status(status) => formatter.format_simple(status, writer),
            CommandPayload::ListServices(list) => {
                use crate::display::CompactTableFormattable;
                // Show detailed view only when explicitly requested
                if options.detailed || options.verbose {
                    list.format_detailed(&formatter, writer)
                } else {
                    // Use compact table format by default
                    list.format_compact_table(&formatter, writer)
                }
            },
            CommandPayload::GitLog(log) => {
                use crate::display::CompactTableFormattable;
                // Git log should show detailed view unless compact requested
                if options.detailed || options.verbose {
                    log.format_detailed(&formatter, writer)
                } else {
                    log.format_compact_table(&formatter, writer)
                }
            },
            CommandPayload::GitBranches(branches) => formatter.format_simple(branches, writer),
            CommandPayload::GitStatus(status) => formatter.format_simple(status, writer),
            CommandPayload::ServiceConfig(config) => formatter.format_simple(config, writer),
            CommandPayload::Dependencies(deps) => formatter.format_simple(deps, writer),
            CommandPayload::Error(error) => formatter.format_simple(error, writer),
            CommandPayload::Stdout(output) => write!(writer, "{}", output),
            CommandPayload::Empty => Ok(()),
            _ => {
                // For types that don't have SimpleFormattable yet, fall back to serialization
                formatter.format_serializable(self, writer)
            }
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
    use super::{colors, DisplayOptions};
    use std::io::{self, Write};

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
        _options: &DisplayOptions,
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