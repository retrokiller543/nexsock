# Nexsock

[![Pre-release](https://img.shields.io/badge/status-pre--release-orange)]()

A powerful CLI tool for managing your local development environment.

## ⚠️ Pre-release Notice

This project is currently in pre-release status. Please note:
- Breaking changes may occur between updates
- No backward compatibility is guaranteed before version 1.0.0
- Features and APIs may change significantly

## Overview

Nexsock simplifies local development environment management through a CLI interface. It consists of two main components:
- `nexsock`: The command-line interface
- `nexsockd`: The background daemon service

Communication between these components occurs via:
- Unix sockets (Linux/macOS)
- TCP sockets (Windows)

## Features

- Service lifecycle management (start, stop, restart)
- Service dependency handling
- Configuration management
- Git operations integration
- Cross-platform support (Windows, Linux, macOS)

## Installation

### Pre-built Binaries

Download the latest pre-built binaries from our [releases page](https://github.com/retrokiller543/nexsock/releases).

### Building from Source

Prerequisites:
- [Rust](https://www.rust-lang.org/) (latest stable version)
- Git

Steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/retrokiller543/nexsock.git
   cd nexsock
   ```

2. Build the project:
   ```bash
   # Debug build
   cargo build

   # Release build (recommended for production use)
   cargo build --release
   ```

The compiled binaries will be available in:
- Debug build: `target/debug/`
- Release build: `target/release/`

## Usage

### CLI Reference

```bash
nexsock [OPTIONS] <COMMAND>

Commands:
  start       Start a service
  stop        Stop a service
  restart     Restart a service
  list        List all services
  status      Get status of a service
  add         Add a new service
  remove      Remove a service
  config      Update service configuration
  dependency  Manage service dependencies
  git         Git operations
  help        Print this message or the help of given subcommand(s)

Options:
  -s, --socket <SOCKET>  Socket path [default: /tmp/nexsockd.sock]
  -h, --help            Print help information
  -V, --version         Print version information
```

### Basic Examples

Start a service:
```bash
nexsock start service-name
```

Check service status:
```bash
nexsock status service-name
```

Add a new service:
```bash
nexsock add <service-name> <repo-url> <repo-path> <port>
```

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- [Open an issue](https://github.com/retrokiller543/nexsock/issues)

## Roadmap

The roadmap has not yet been decided and therefore is open for suggestion and feedback

- [ ] Service health monitoring
- [ ] TUI interface?
- [ ] Web interface?
- [ ] Configuration profiles
- [ ] Service templates?