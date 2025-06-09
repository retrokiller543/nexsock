# Nexsock Display System

A highly scalable and extensible CLI output formatting system that makes it trivial to add beautiful formatting for any data type.

## Architecture Overview

### Core Design Principles
- **Simplicity**: Adding a new format should be as easy as implementing one trait
- **Extensibility**: New output formats can be added without touching existing code
- **Consistency**: All data types get the same formatting options automatically
- **Performance**: Zero-cost abstractions, enum-based dispatch

### Key Components

#### `SimpleFormatter` 
An enum-based formatter that dispatches to different output formats:
```rust
pub enum SimpleFormatter {
    Json { pretty: bool },
    Yaml,
    Table { use_color: bool, show_headers: bool },
    Plain,
}
```

#### `SimpleFormattable` Trait
The core trait that data types implement to get beautiful formatting:
```rust
pub trait SimpleFormattable {
    fn to_key_value_pairs(&self) -> Vec<(String, String)>;
    fn title(&self) -> Option<String> { None }
}
```

#### `CliDisplay` Trait
High-level trait for CLI display that uses the formatter system:
```rust
pub trait CliDisplay {
    fn display(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()>;
}
```

## Adding New Data Types

Adding beautiful formatting for any data type is incredibly simple:

```rust
#[derive(Debug, Serialize)]
struct MyServiceInfo {
    name: String,
    port: u16,
    status: String,
}

impl SimpleFormattable for MyServiceInfo {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("Service Name".to_string(), self.name.clone()),
            ("Port".to_string(), self.port.to_string()),
            ("Status".to_string(), self.status.clone()),
        ]
    }
    
    fn title(&self) -> Option<String> {
        Some(format!("Service: {}", self.name))
    }
}
```

That's it! Your type now automatically supports:
- ✅ **JSON** output (pretty-printed)
- ✅ **YAML** output
- ✅ **Table** output (with colors and headers)
- ✅ **Plain** text output

## Output Format Examples

### Table Format
```
Service: my-api
─────────────────────
Field          Value
─────────────────────
Service Name   my-api
Port          8080
Status        Running
```

### JSON Format
```json
{
  "name": "my-api",
  "port": 8080,
  "status": "Running"
}
```

### Plain Format
```
Service Name: my-api
Port: 8080
Status: Running
```

## Adding New Output Formats

Adding a completely new output format is straightforward:

```rust
impl SimpleFormatter {
    pub fn format_csv<T>(&self, data: &T, writer: &mut dyn Write) -> io::Result<()>
    where
        T: SimpleFormattable,
    {
        let pairs = data.to_key_value_pairs();
        writeln!(writer, "Field,Value")?;
        for (key, value) in pairs {
            writeln!(writer, "{},{}", key, value)?;
        }
        Ok(())
    }
}
```

## Current Implementations

The system currently includes formatters for:

### Protocol Types
- `ServiceStatus` - Complete service information with dependencies
- `ListServicesResponse` - Service overview with statistics  
- `ErrorPayload` - Error information with codes and details
- `GitLogResponse` - Git commit history (via macro)
- `GitListBranchesResponse` - Git branch listings (via macro)

### Helper Macro
For simple types, use the convenience macro:
```rust
impl_simple_formattable!(MyType, "My Type Title");
```

## CLI Integration

The system integrates seamlessly with CLI arguments:
- `--format table|json|yaml|plain` - Choose output format
- `--no-headers` - Disable table headers
- `--color/--no-color` - Control color output
- `--verbose` - Enable detailed output

## Migration from Old System

The old complex trait-based system has been replaced with this simpler approach:

### Before (Complex)
```rust
impl Display for MyType {
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> { /* ... */ }
    fn display_json(&self, writer: &mut dyn Write) -> io::Result<()> { /* ... */ }
    fn display_yaml(&self, writer: &mut dyn Write) -> io::Result<()> { /* ... */ }  
    fn display_plain(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> { /* ... */ }
}
```

### After (Simple)
```rust
impl SimpleFormattable for MyType {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> { /* ... */ }
}
```

## Performance Characteristics

- **Zero allocation** for format selection (enum dispatch)
- **Minimal allocations** for string formatting
- **No trait objects** or dynamic dispatch overhead
- **Compile-time optimization** of format-specific code paths

## Testing

The system includes comprehensive examples and tests in:
- `examples.rs` - Usage examples and extensions
- Built-in test coverage for all protocol types
- Documentation tests for public APIs

## Future Extensions

The architecture easily supports:
- **Custom table libraries** (tabled, comfy-table, etc.)
- **Rich formatting** (colors, icons, progress bars)
- **Interactive output** (for TUI interfaces)
- **Streaming output** (for large datasets)
- **Format plugins** (external format providers)

This design makes nexsock's CLI output system both powerful and incredibly easy to extend!