use crate::display::{colors, utils, DisplayOptions, OutputFormat};
use std::io::{self, Write};

/// A simple, enum-based formatter that's easy to extend
#[derive(Debug)]
pub enum SimpleFormatter {
    Json { pretty: bool },
    Yaml,
    Table { use_color: bool, show_headers: bool },
    Plain,
}

impl SimpleFormatter {
    /// Create a formatter from display options
    pub fn from_options(options: &DisplayOptions) -> Self {
        match options.format {
            OutputFormat::Json => SimpleFormatter::Json { pretty: true },
            OutputFormat::Yaml => SimpleFormatter::Yaml,
            OutputFormat::Table => SimpleFormatter::Table {
                use_color: options.color,
                show_headers: !options.no_headers,
            },
            OutputFormat::Plain => SimpleFormatter::Plain,
        }
    }
    
    /// Format any serializable data
    pub fn format_serializable<T>(&self, data: &T, writer: &mut dyn Write) -> io::Result<()>
    where
        T: serde::Serialize,
    {
        match self {
            SimpleFormatter::Json { pretty } => {
                let json = if *pretty {
                    serde_json::to_string_pretty(data)
                } else {
                    serde_json::to_string(data)
                }
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                writeln!(writer, "{}", json)
            }
            SimpleFormatter::Yaml => {
                let yaml = serde_yaml::to_string(data)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                write!(writer, "{}", yaml)
            }
            _ => {
                // For table and plain, we need specific implementations
                writeln!(writer, "Table/Plain formatting requires specific implementation")
            }
        }
    }
    
    /// Format a simple key-value display
    pub fn format_key_value(&self, pairs: &[(&str, &str)], writer: &mut dyn Write) -> io::Result<()> {
        match self {
            SimpleFormatter::Json { .. } => {
                let map: std::collections::HashMap<&str, &str> = pairs.iter().cloned().collect();
                self.format_serializable(&map, writer)
            }
            SimpleFormatter::Yaml => {
                let map: std::collections::HashMap<&str, &str> = pairs.iter().cloned().collect();
                self.format_serializable(&map, writer)
            }
            SimpleFormatter::Table { use_color, show_headers } => {
                let dummy_options = DisplayOptions {
                    format: OutputFormat::Table,
                    verbose: false,
                    no_headers: !show_headers,
                    color: *use_color,
                    detailed: false,
                };
                
                if *show_headers {
                    let field_header = utils::colorize("Field", colors::BOLD, &dummy_options);
                    let value_header = utils::colorize("Value", colors::BOLD, &dummy_options);
                    writeln!(writer, "{:<20} {}", field_header, value_header)?;
                    if *use_color {
                        writeln!(writer, "{}", utils::colorize("─".repeat(50).as_str(), colors::DIM, &dummy_options))?;
                    }
                }
                for (key, value) in pairs {
                    let colored_key = utils::colorize(key, colors::CYAN, &dummy_options);
                    writeln!(writer, "{:<20} {}", colored_key, value)?;
                }
                Ok(())
            }
            SimpleFormatter::Plain => {
                for (key, value) in pairs {
                    writeln!(writer, "{}: {}", key, value)?;
                }
                Ok(())
            }
        }
    }
    
    /// Format a list of items
    pub fn format_list(&self, items: &[&str], writer: &mut dyn Write) -> io::Result<()> {
        match self {
            SimpleFormatter::Json { .. } => self.format_serializable(&items, writer),
            SimpleFormatter::Yaml => self.format_serializable(&items, writer),
            SimpleFormatter::Table { show_headers, .. } => {
                if *show_headers {
                    writeln!(writer, "Items")?;
                    writeln!(writer, "-----")?;
                }
                for item in items {
                    writeln!(writer, "{}", item)?;
                }
                Ok(())
            }
            SimpleFormatter::Plain => {
                for item in items {
                    writeln!(writer, "{}", item)?;
                }
                Ok(())
            }
        }
    }
    
    /// Format a compact table with columns
    pub fn format_table(&self, headers: &[&str], rows: &[Vec<String>], writer: &mut dyn Write) -> io::Result<()> {
        match self {
            SimpleFormatter::Json { .. } => {
                // Convert to array of objects for JSON
                let objects: Vec<std::collections::HashMap<&str, &str>> = rows.iter()
                    .map(|row| {
                        headers.iter().zip(row.iter())
                            .map(|(h, v)| (*h, v.as_str()))
                            .collect()
                    })
                    .collect();
                self.format_serializable(&objects, writer)
            }
            SimpleFormatter::Yaml => {
                // Convert to array of objects for YAML
                let objects: Vec<std::collections::HashMap<&str, &str>> = rows.iter()
                    .map(|row| {
                        headers.iter().zip(row.iter())
                            .map(|(h, v)| (*h, v.as_str()))
                            .collect()
                    })
                    .collect();
                self.format_serializable(&objects, writer)
            }
            SimpleFormatter::Table { use_color, show_headers } => {
                let dummy_options = DisplayOptions {
                    format: OutputFormat::Table,
                    verbose: false,
                    no_headers: !show_headers,
                    color: *use_color,
                    detailed: false,
                };
                
                if rows.is_empty() {
                    writeln!(writer, "No items found")?;
                    return Ok(());
                }
                
                // Calculate column widths
                let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
                for row in rows {
                    for (i, cell) in row.iter().enumerate() {
                        if i < widths.len() {
                            widths[i] = widths[i].max(cell.len());
                        }
                    }
                }
                
                // Print headers
                if *show_headers {
                    for (i, &header) in headers.iter().enumerate() {
                        if i > 0 { write!(writer, "  ")?; }
                        let colored_header = utils::colorize(header, colors::BOLD, &dummy_options);
                        write!(writer, "{:<width$}", colored_header, width = widths[i])?;
                    }
                    writeln!(writer)?;
                    
                    if *use_color {
                        for (i, &width) in widths.iter().enumerate() {
                            if i > 0 { write!(writer, "  ")?; }
                            write!(writer, "{}", utils::colorize("─".repeat(width).as_str(), colors::DIM, &dummy_options))?;
                        }
                        writeln!(writer)?;
                    }
                }
                
                // Print rows
                for row in rows {
                    for (i, cell) in row.iter().enumerate() {
                        if i > 0 { write!(writer, "  ")?; }
                        
                        // Apply status coloring for status column
                        let colored_cell = if i < headers.len() && headers[i].to_lowercase() == "status" {
                            utils::colorize(cell, utils::status_color(cell), &dummy_options)
                        } else {
                            cell.clone()
                        };
                        
                        write!(writer, "{:<width$}", colored_cell, width = widths.get(i).unwrap_or(&0))?;
                    }
                    writeln!(writer)?;
                }
                Ok(())
            }
            SimpleFormatter::Plain => {
                for row in rows {
                    let line = headers.iter().zip(row.iter())
                        .map(|(h, v)| format!("{}: {}", h, v))
                        .collect::<Vec<_>>()
                        .join(", ");
                    writeln!(writer, "{}", line)?;
                }
                Ok(())
            }
        }
    }
}

/// Trait for types that can be formatted with SimpleFormatter
pub trait SimpleFormattable {
    /// Convert to key-value pairs for display
    fn to_key_value_pairs(&self) -> Vec<(String, String)>;
    
    /// Get a simple string representation
    fn to_simple_string(&self) -> String
    where
        Self: std::fmt::Debug,
    {
        format!("{:?}", self)
    }
    
    /// Optional: provide a title for this data
    fn title(&self) -> Option<String> {
        None
    }
}

/// Trait for types that can be formatted as compact tables
pub trait CompactTableFormattable {
    /// Get table headers
    fn table_headers(&self) -> Vec<&'static str>;
    
    /// Get table rows
    fn table_rows(&self) -> Vec<Vec<String>>;
    
    /// Optional: provide a title for this table
    fn table_title(&self) -> Option<String> {
        None
    }
    
    /// Format as a compact table
    fn format_compact_table(&self, formatter: &SimpleFormatter, writer: &mut dyn Write) -> io::Result<()> {
        if let Some(title) = self.table_title() {
            writeln!(writer, "{}", title)?;
            writeln!(writer)?;
        }
        
        let headers = self.table_headers();
        let rows = self.table_rows();
        formatter.format_table(&headers, &rows, writer)
    }
}

/// Trait for collections that can show both summary and detailed views
pub trait CollectionFormattable: SimpleFormattable {
    type Item: SimpleFormattable + serde::Serialize;
    
    /// Get the items in this collection
    fn items(&self) -> &[Self::Item];
    
    /// Format as a detailed list showing each item
    fn format_detailed(&self, formatter: &SimpleFormatter, writer: &mut dyn Write) -> io::Result<()> {
        if self.items().is_empty() {
            writeln!(writer, "No items found")?;
            return Ok(());
        }
        
        if let Some(title) = self.title() {
            writeln!(writer, "{}", title)?;
            writeln!(writer)?;
        }
        
        for (i, item) in self.items().iter().enumerate() {
            if i > 0 {
                writeln!(writer)?;
            }
            formatter.format_simple(item, writer)?;
        }
        Ok(())
    }
    
    /// Whether to show detailed view by default
    fn should_show_detailed(&self) -> bool {
        false // Changed default to prefer compact view
    }
}

/// Extension to SimpleFormatter for SimpleFormattable types
impl SimpleFormatter {
    pub fn format_simple<T>(&self, data: &T, writer: &mut dyn Write) -> io::Result<()>
    where
        T: SimpleFormattable + serde::Serialize,
    {
        match self {
            SimpleFormatter::Json { .. } | SimpleFormatter::Yaml => {
                self.format_serializable(data, writer)
            }
            SimpleFormatter::Table { .. } | SimpleFormatter::Plain => {
                if let Some(title) = data.title() {
                    writeln!(writer, "{}", title)?;
                    writeln!(writer)?;
                }
                
                let pairs = data.to_key_value_pairs();
                let pair_refs: Vec<(&str, &str)> = pairs.iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                self.format_key_value(&pair_refs, writer)
            }
        }
    }
}