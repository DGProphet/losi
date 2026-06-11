# Modern LiteStep

**Desktop Customization Engine for Windows 11** - A modern successor to the legendary LiteStep desktop shell.

## Vision

Restore the power of desktop customization to Windows 11. LiteStep was a revolutionary desktop shell that allowed users to completely customize their Windows experience. This project brings that capability to modern Windows using Rust and Windows 11 APIs.

## Modes

### Explorer Integration (Phase 1) ✨
Non-destructive integration with Windows Explorer:
- Apply custom themes without replacing shell
- Desktop widgets and overlays
- Hotspot detection
- Safe to use alongside Windows UI
- Easier to debug and develop

### Shell Replacement (Phase 2) 🔥
Full shell replacement like original LiteStep:
- Replace explorer.exe with losi.exe
- Complete desktop control
- Custom taskbar, menus, and window decorations
- Similar to Windows 9x LiteStep experience

## Features

### Current
- ✅ Theme configuration system
- ✅ .rc file parser (LiteStep format)
- ✅ Configuration management
- ✅ Modular architecture

### In Development
- 🔄 Explorer integration
- 🔄 Theme application
- 🔄 Desktop rendering
- 🔄 Widget system

### Planned
- 📋 Custom window decorations
- 📋 Taskbar replacement
- 📋 Global hotkeys
- 📋 Widget library
- 📋 Plugin system
- 📋 Live theme reload
- 📋 Shell replacement mode

## Installation

### From Source

```bash
git clone https://github.com/DGProphet/losi.git
cd losi
git checkout hybrid-shell-win11
cargo build --release
```

Binary: `target/release/losi.exe`

## Usage

### Initialize Configuration

```bash
losi init
```

Creates configuration at `%APPDATA%\losi\config.toml`

### Create a Theme

Create a `.rc` file with LiteStep format:

```ini
[Desktop]
Background=C:\Users\YourName\Pictures\background.png
Color=0x1F1F1F

[Taskbar]
Height=48
Opacity=0.95

[Widgets]
Clock=enabled
Weather=enabled
```

### Apply a Theme

```bash
losi theme apply C:\Themes\mytheme.rc
```

### Start Customization

```bash
# Explorer integration (default)
losi start

# Shell replacement mode (future)
losi start --shell
```

## Project Structure

```
src/
├── main.rs              # Entry point & CLI
├── config/              # Configuration management
│   ├── mod.rs          # Config structure
│   ├── init.rs         # Initialization
│   └── commands.rs     # Config commands
├── theme/              # Theme engine
│   ├── mod.rs          # Theme management
│   └── parser.rs       # .rc parser
├── desktop/            # Desktop management
│   ├── mod.rs          # Desktop manager
│   ├── widgets.rs      # Widget system
│   └── rendering.rs    # Desktop rendering
└── explorer/           # Explorer integration
    ├── mod.rs          # Integration control
    ├── hooking.rs      # Process hooking
    └── integration.rs  # Explorer hooks
```

## Configuration

Default location: `%APPDATA%\losi\config.toml`

```toml
[desktop]
enabled = true
show_widgets = true
widget_opacity = 0.9
animation_speed = 200

[theme]
current_theme = "default"
theme_directory = "C:\\Users\\YourName\\AppData\\Roaming\\losi\\themes"
auto_reload = true

[system]
integration_mode = "explorer"  # or "shell"
auto_start = false
log_level = "info"
```

## Development

### Building

```bash
cargo build              # Debug
cargo build --release   # Release (optimized)
```

### Running with Logging

```bash
RUST_LOG=debug losi start
RUST_LOG=trace losi start
```

### Testing

```bash
cargo test
```

## Roadmap

### Phase 1: Explorer Integration
- [ ] Process hooking into explorer.exe
- [ ] Theme rendering system
- [ ] Desktop widget rendering
- [ ] Mouse/click detection
- [ ] Window decoration hooks

### Phase 2: Full Customization
- [ ] Custom taskbar implementation
- [ ] Window frame customization
- [ ] System menu replacement
- [ ] Icon themes
- [ ] Cursor themes

### Phase 3: Shell Replacement
- [ ] Replace explorer.exe
- [ ] Desktop management
- [ ] Window manager
- [ ] File browser integration
- [ ] System tray

### Phase 4: Advanced Features
- [ ] Plugin system (WASM)
- [ ] Macro system
- [ ] Scripting support
- [ ] Community theme hub
- [ ] Live preview editor

## History

LiteStep was a legendary Windows desktop shell that dominated the customization scene from the 1990s through early 2000s. It allowed complete customization of the desktop through powerful theme files and a robust plugin system. This project aims to bring that experience to modern Windows 11.

## License

MIT License - See LICENSE file for details

## Contributing

Contributions welcome! This is an ambitious project to resurrect desktop customization.

## Acknowledgments

- **Original LiteStep** - The inspiration and foundation
- **Rust Community** - Amazing tools and libraries
- **Windows API** - Making modern customization possible
