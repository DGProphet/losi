# Hybrid Shell for Windows 11

A modern, feature-rich command shell for Windows 11 with native Unix/Linux interoperability through WSL2 integration.

## Features

- **Modern CLI Interface** - Built with Rust for performance and reliability
- **Windows 11 Integration** - Native Windows 11 features and Windows Terminal support
- **WSL2 Bridge** - Seamless execution of Linux commands directly from the shell
- **Customizable Themes** - Multiple built-in themes (default, dracula, nord, solarized)
- **Command History** - Persistent command history with search
- **Environment Management** - Easy environment variable management
- **Async/Await Support** - Non-blocking operations for better responsiveness
- **Fluent Design** - Modern Windows 11 aesthetics

## Getting Started

### Prerequisites

- Windows 11
- Rust 1.70+ (for development)
- WSL2 (optional, for Linux command integration)

### Installation

#### From Source

```bash
git clone https://github.com/DGProphet/losi.git
cd losi
git checkout hybrid-shell-win11
cargo build --release
```

The binary will be available at `target/release/hshell.exe`

#### From Releases

(Coming soon)

### Quick Start

```bash
# Initialize configuration
hshell init

# Start the interactive shell
hshell

# Check WSL2 status
hshell wsl status

# List available themes
hshell theme list

# Apply a theme
hshell theme apply dracula
```

## Commands

### Core Commands

```bash
hshell init                          # Initialize default configuration
hshell config                        # Show current configuration
hshell config --path                 # Show config file location
```

### Theme Management

```bash
hshell theme list                    # List available themes
hshell theme apply <name>            # Apply a theme
hshell theme current                 # Show currently active theme
```

### WSL2 Integration

```bash
hshell wsl status                    # Check WSL2 installation status
hshell wsl list                      # List installed distributions
hshell wsl set-default <distro>      # Set default distribution
```

### Environment Variables

```bash
hshell env list                      # List all environment variables
hshell env get <name>                # Get specific variable
hshell env set <name> <value>        # Set a variable
```

### Interactive Mode

Once in the shell, use standard Windows commands:

```
hshell> dir
hshell> cd C:\
hshell> python --version
hshell> wsl ls -la
hshell> exit
```

## Configuration

Configuration is stored in TOML format at:
- **Windows:** `%APPDATA%\hshell\config.toml`
- **Default:** Creates automatically on first run

### Example Configuration

```toml
[theme]
name = "default"

[theme.colors]
foreground = "#E1E1E1"
background = "#0C0C0C"
accent = "#007ACC"
error = "#F48771"
success = "#4EC9B0"

[shell]
prompt = "hshell> "
editor = "code"
shell_type = "powershell"

[wsl2]
enabled = false
default_distro = "Ubuntu"
path_mapping = true

[history]
enabled = true
max_entries = 10000
```

## Architecture

### Project Structure

```
src/
├── main.rs           # CLI entry point
├── config/           # Configuration management
├── shell/            # Core shell logic
├── windows/          # Windows 11 integration
├── wsl2/             # WSL2 bridge
├── cli/              # CLI utilities
└── history/          # Command history
```

### Key Components

- **Shell** - Main interactive shell loop and command execution
- **Config** - TOML-based configuration with defaults
- **Windows** - Windows 11 specific features and terminal detection
- **WSL2** - Bridge for executing Linux commands via WSL2
- **History** - Persistent command history with JSON storage

## Development

### Building

```bash
cargo build              # Debug build
cargo build --release   # Release build with optimizations
```

### Running Tests

```bash
cargo test
```

### Running with Logging

```bash
RUST_LOG=debug hshell
RUST_LOG=trace hshell -vvv
```

## Roadmap

- [ ] Tab completion
- [ ] Plugin system (WASM-based)
- [ ] Git integration
- [ ] Package manager support
- [ ] Custom keybindings
- [ ] Split panes / multi-window support
- [ ] Command aliases
- [ ] Advanced history search
- [ ] Windows Terminal integration API
- [ ] Theming system enhancement

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - See LICENSE file for details

## Acknowledgments

- Originally evolved from [LOSI](https://github.com/Tobbe/losi) - LiteStep OpenSource Installer
- Built with [Rust](https://www.rust-lang.org/)
- Uses [Tokio](https://tokio.rs/) for async runtime
- CLI framework by [Clap](https://docs.rs/clap/)
