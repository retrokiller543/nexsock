# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# important-instruction-reminders
Do what has been asked; nothing more, nothing less.
NEVER create files unless they're absolutely necessary for achieving your goal.
ALWAYS prefer editing an existing file to creating a new one.
NEVER proactively create documentation files (*.md) or README files. Only create documentation files if explicitly requested by the User.

# Nexsock Architecture Documentation

## Overview

Nexsock is a powerful CLI tool and daemon system for managing local development services. It provides a comprehensive service lifecycle management platform with support for dependencies, configuration management, Git operations, and extensibility through plugins.

The system follows a client-server architecture where the daemon (`nexsockd`) manages services and the CLI client (`nexsock`) communicates with the daemon via Unix/TCP sockets.

## Project Structure

### Workspace Organization

Nexsock is organized as a Cargo workspace with the following crates:

```
nexsock/
├── src/                        # Main daemon source code
├── nexsock/                    # CLI client
├── nexsock-abi/               # Plugin ABI definitions  
├── nexsock-client/            # Client library for daemon communication
├── nexsock-config/            # Configuration management
├── nexsock-db/                # Database layer (SeaORM)
│   ├── migration/             # Database migrations
│   └── src/                   # Models and repositories
├── nexsock-example-plugin/    # Example plugin implementation
├── nexsock-plugins/           # Plugin system (Lua + Native)
├── nexsock-protocol/          # Communication protocol definitions
├── nexsock-testing/           # Testing infrastructure and utilities
├── nexsock-utils/             # Shared utilities
└── nexsock-web/               # Web interface (Axum)
```

### Key Dependencies

- **Database**: SeaORM with SQLite backend
- **Async Runtime**: Tokio with full features
- **IPC**: Unix sockets (Linux/macOS), TCP sockets (Windows)
- **Serialization**: Bincode for protocol communication
- **Web Framework**: Axum with Tera templating
- **Plugin System**: MLua for Lua plugins, savefile-abi for native plugins
- **Memory Allocator**: tikv-jemallocator (optional)

## Core Architecture

### 1. Trait-Based Design

The daemon implements a comprehensive trait system that abstracts core functionality:

#### Primary Management Traits

**ServiceManagement** (extends ProcessManager)
- Service lifecycle operations (start, stop, restart)
- Service registration and removal
- Status monitoring and log retrieval
- High-level service abstractions

**ConfigurationManagement**
- Service configuration updates
- Configuration file format handling
- Run command management

**DependencyManagement** 
- Service dependency relationships
- Dependency ordering for startup
- Tunneling support between services

**ProcessManager**
- Low-level process spawning and management
- Resource cleanup and port management
- Process monitoring and logging

#### Git Operations

**GitManagement**
- Repository cloning and updates
- Branch management and authentication

**GitBackend**
- Abstraction over Git implementations
- Currently supports system Git

**GitService**
- High-level Git service operations
- Integration with service management

#### Utility Traits

**FromDbResult\<T\>**
- Converts database results between collection types
- Handles Option\<T\> ↔ Vec\<T\> conversions

**VecExt\<T\>**
- Safe vector operations
- Non-panicking element removal

### 2. Database Schema

The system uses SeaORM with SQLite for persistence:

#### Core Tables

**service**
- `id`: Primary key
- `name`: Unique service name
- `repo_url`: Git repository URL
- `repo_path`: Local filesystem path
- `port`: Service port number
- `status`: Current state (Starting/Running/Stopped/Failed)
- `config_id`: Foreign key to service_config

**service_config**
- `id`: Primary key
- `filename`: Configuration file name
- `format`: Configuration format (Env/Properties)
- `run_command`: Command to execute the service

**service_dependency**
- `id`: Primary key
- `service_id`: Service that has the dependency
- `dependent_service_id`: Service that is depended upon
- `tunnel_enabled`: Whether tunneling is enabled

#### Migration System

Uses SeaORM migration framework:
- Migrations in `nexsock-db/migration/src/`
- CLI tool for migration management
- Schema versioning and rollback support

### 3. Communication Protocol

#### Protocol Design

The communication layer uses a binary protocol over Unix/TCP sockets:

**Header Structure**
- Fixed-size header with command type and payload length
- Versioned protocol for backward compatibility

**Command System**
- Enum-based command dispatch
- Strongly typed payloads using bincode serialization
- Request-response pattern

#### Key Commands

**Service Management**
- `AddService`: Register a new service
- `StartService`: Start a service with environment variables
- `StopService`: Stop a running service
- `ListServices`: Get all services with status
- `ServiceStatus`: Get detailed service information

**Configuration**
- `UpdateConfig`: Update service configuration
- `GetConfig`: Retrieve service configuration

**Dependencies**
- `AddDependency`: Create service dependency
- `RemoveDependency`: Remove dependency relationship
- `ListDependencies`: Get service dependencies

**Git Operations**
- `GitClone`: Clone repository
- `GitPull`: Update repository
- `GitStatus`: Get repository status

### 4. Plugin System

#### Architecture

Dual plugin system supporting:

**Native Plugins**
- Rust-based plugins using savefile-abi
- Type-safe ABI with versioning
- Compiled shared libraries

**Lua Plugins**
- Scripted plugins using MLua runtime
- Dynamic loading and execution
- Sandbox environment

#### Plugin Interface

**PreHook Trait**
- `pre_command`: Intercept commands before processing
- `pre_start_command`: Specialized hook for service starts
- Plugin lifecycle management

#### Plugin Directory Structure

```
~/.config/nexsock/plugins/
├── native/          # Native shared libraries
└── lua/             # Lua script files
```

### 5. Web Interface

#### Technology Stack

**Backend**
- Axum web framework
- Tera templating engine
- REST API endpoints

**Frontend**
- HTMX for dynamic interactions
- Embedded static assets
- Server-side rendering

#### API Endpoints

**Service Management**
- `GET /services` - List all services
- `GET /services/{id}` - Get service details
- `POST /services` - Add new service
- `POST /services/{id}/start` - Start service
- `POST /services/{id}/stop` - Stop service
- `DELETE /api/services/{id}` - Remove service

**Static Assets**
- Embedded using rust-embed
- Compression and caching layers
- MIME type detection

## Development Workflow

### Build Commands

```bash
# Debug build
cargo build

# Release build  
cargo build --release

# Build with jemalloc allocator
cargo build --features jemalloc

# Run tests
cargo test

# Run specific crate tests
cargo test -p nexsock-db

# Run single test by name
cargo test test_name

# Run tests with output (especially useful for debugging)
cargo test -- --nocapture

# Run tests in specific file/module
cargo test -p nexsock-db repository::test_service_crud
```

### Database Operations

```bash
# Run migrations
cd nexsock-db/migration && cargo run

# Generate new migration
cd nexsock-db/migration && cargo run -- generate MIGRATION_NAME

# Migration status
cd nexsock-db/migration && cargo run -- status

# Rollback migrations
cd nexsock-db/migration && cargo run -- down -n 1
```

### Development Server

```bash
# Start daemon
cargo run --bin nexsockd

# Start web interface  
cargo run --bin nexsock-web

# Use CLI
cargo run --bin nexsock -- help
```

### Web Interface Development

The web interface (`nexsock-web`) includes TypeScript/TSX support with component-based architecture:

```bash
# Build TypeScript/TSX to JavaScript (production)
cd nexsock-web && bun run build

# Build for development (with sourcemaps)
cd nexsock-web && bun run build:dev

# Watch mode for development
cd nexsock-web && bun run watch

# Type check without building
cd nexsock-web && bun run check

# Build with type checking
cd nexsock-web && bun run build-check
```

**Web Tech Stack:**
- **Build Tool**: Bun with TypeScript compilation
- **Frontend Framework**: TSX with custom createElement function
- **Dynamic Interactions**: HTMX
- **CSS**: Scoped component styles with automatic class generation
- **Component Architecture**: Modular components in `src-ts/components/`

### CI/CD Pipeline

**GitHub Actions** (`/.github/workflows/test.yml`):
- **Platform Matrix**: Linux (x86_64, aarch64), macOS (x86_64, aarch64), Windows (x86_64)
- **Feature Testing**: Default + jemalloc feature combinations
- **Quality Gates**: `cargo fmt`, `cargo clippy`, security audit (`cargo-audit`)
- **Cross-compilation**: Automated toolchain setup

**Quality Commands**
```bash
# Code formatting
cargo fmt --all -- --check

# Linting 
cargo clippy --all-targets --all-features

# Security audit
cargo audit

# Run all quality checks
cargo fmt --all -- --check && cargo clippy --all-targets --all-features && cargo audit
```

### Distribution

Uses cargo-dist for cross-platform releases:
- GitHub Actions CI/CD
- Multiple target platforms (x86_64, aarch64)
- Windows MSI, macOS, Linux packages
- Homebrew formula generation

## Testing Strategy

### Testing Infrastructure (nexsock-testing crate)

**Comprehensive Testing Framework**
- **Testing Macros**: `test_async!`, `test_with_db!`, `test_with_fixtures!`
- **Database Testing**: In-memory SQLite with full migrations
- **Service Fixtures**: Builder pattern for test data creation (`ServiceFixtureBuilder`)
- **Mock Framework**: Process managers, protocol handlers, daemon state
- **Cross-platform Process Testing**: Command execution with timeouts

**Test Structure**
```rust
// Common test utilities
pub async fn setup_in_memory_db() -> DatabaseConnection {
    // Creates :memory: SQLite database
    // Runs migrations automatically
}

// Fixture builders for consistent test data
let service = ServiceFixtureBuilder::new()
    .with_name("test-service")
    .with_port(8080)
    .build(&db)
    .await?;
```

### Database Testing

**In-Memory SQLite**
- Fast test execution
- Isolated test environments
- Full migration testing

**Repository Testing**
- Unit tests for each repository
- CRUD operation validation
- Constraint testing

### Integration Testing

**Service Management**
- End-to-end service lifecycle tests
- Dependency resolution testing
- Error condition handling

**Protocol Testing**
- Command serialization/deserialization
- Client-server communication
- Error propagation

### Plugin Testing

**Native Plugin ABI**
- Type safety validation
- Version compatibility testing
- Loading and unloading cycles

**Lua Plugin Execution**
- Script execution sandboxing
- Error handling and recovery
- Resource limitation testing

## Configuration Management

### Configuration Sources

**Static Configuration**
- Compile-time defaults
- Feature flag configuration

**Environment Variables**
- `NEXSOCK_CONFIG_DIR` - Configuration directory
- `PLUGINS_DIR` - Plugin directory override
- `DATABASE_URL` - Database connection override

**Configuration Structure**
```rust
pub struct NexsockConfig {
    pub database: DatabaseConfig,
    pub daemon: DaemonConfig,
    pub plugins: PluginConfig,
}
```

## Security Considerations

### Process Isolation
- Services run in separate processes
- Resource cleanup on termination
- Process group management

### Plugin Security
- Lua sandbox execution
- Native plugin ABI versioning
- Plugin directory permissions

### Network Security
- Unix socket permissions
- TCP socket binding controls
- Service port validation

## Performance Optimizations

### Database Performance
- Connection pooling (5-21 connections)
- Query optimization with indexes
- Efficient foreign key relationships

### Memory Management
- Optional jemalloc allocator
- Circular log buffers for service output
- Lazy static initialization patterns

### Concurrency
- Tokio async runtime
- DashMap for concurrent access
- Parking lot synchronization primitives

## Error Handling

### Error Types

**Database Errors**
- SeaORM error propagation
- Transaction rollback handling
- Connection failure recovery

**Service Errors**
- Process spawn failures
- Port conflict resolution
- Dependency cycle detection

**Protocol Errors**
- Serialization failures
- Connection timeouts
- Invalid command handling

### Error Propagation

**Current State**: Heavy use of `anyhow::Error` as catch-all throughout the stack.

**Recommended Approach**: Prefer typed errors with `thiserror::Error` for:
- Explicit error handling and better API design
- Specific error variants that callers can handle programmatically  
- Better debugging and testing capabilities
- Proper error serialization for client communication

Only use `anyhow::Error` for prototyping or when error types genuinely cannot be known at compile time.

## Future Architecture Considerations

### Scalability
- Distributed service management
- Remote daemon support
- Service mesh integration

### Extensibility
- Plugin marketplace
- Custom service types
- Advanced dependency models

### Monitoring
- Metrics collection
- Health check endpoints
- Performance profiling integration

## Development Best Practices

### Code Organization
- Trait-based abstractions
- Comprehensive documentation
- Modular crate design

### Testing
- Unit tests for all repositories
- Integration tests for workflows
- Property-based testing for protocols

### Documentation
- Extensive trait documentation
- Architecture decision records
- API documentation generation

This architecture provides a solid foundation for local development service management with room for future expansion and customization through the plugin system.