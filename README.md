# MT5 Terminal - TUI

A Terminal User Interface (TUI) for MetaTrader 5 written in Rust.

## Features

- 📊 View open positions in real-time
- 💰 Monitor account balance, equity, and margin
- ⚡ Lightweight and fast
- 🎨 Color-coded profits (green/red)
- ⌨️ Keyboard-driven interface

## Prerequisites

- **Windows** (MT5 requirement)
- **Rust** toolchain: https://rustup.rs/
- **Python 3.8+**: https://www.python.org/downloads/
- **MT5 Terminal**: https://www.metatrader5.com/
- **MinGW** (for building): Automatically installed via setup

## Quick Start

### 1. Install Dependencies

```bash
# Install Python MT5 library
pip install -r python/requirements.txt
```

### 2. Build the Project

```bash
cargo build --release
```

### 3. Run MT5 Service

Open a terminal and run:

```bash
python python/mt5_service.py
```

This will connect to your MT5 terminal and export data to `data/positions.json` every 5 seconds.

### 4. Run the TUI

In another terminal:

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/mt5-term.exe
```

## Usage

### Keyboard Controls

- `q` or `ESC` - Quit the application
- `Tab` - Switch between views (coming soon)
- `↑↓` - Scroll through positions (coming soon)

### Current Status

**Phase 1 Complete** ✅ 
- Basic TUI skeleton
- Window layout (header, content, footer)
- Keyboard input handling

**Phase 2 In Progress** 🚧
- Data models for positions and account info
- JSON file loading
- Table display for positions

**Phase 3 Planned** 📋
- Python MT5 service integration
- Real-time data updates
- File watching

## Project Structure

```
mt5-term/
├── src/
│   ├── main.rs              # Entry point
│   ├── models/              # Data structures
│   ├── services/            # Data loading, file watching
│   └── widgets/             # UI components
├── python/
│   ├── mt5_service.py       # MT5 data exporter
│   └── requirements.txt     # Python dependencies
├── data/
│   └── positions.json       # Shared data file
└── README.md
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run with cargo
cargo run

# Run tests
cargo test
```

### Note on Windows Builds

This project uses the GNU toolchain (`x86_64-pc-windows-gnu`) instead of MSVC to avoid Visual Studio dependencies. The build system is configured automatically via `.cargo/config.toml`.

## Troubleshooting

### "MT5 initialization failed"

- Ensure MT5 terminal is running
- Check that you're logged into an MT5 account
- Verify Python MT5 library is installed: `pip install MetaTrader5`

### "No such file: positions.json"

- Run the Python service first: `python python/mt5_service.py`
- Check that `data/` directory exists
- Verify MT5 service is connected and writing data

### Build errors on Windows

- Ensure MinGW is installed: `choco install mingw`
- Restart your terminal after installing MinGW
- Try: `rustup target add x86_64-pc-windows-gnu`

## Roadmap

- [x] Phase 1: Basic TUI skeleton
- [ ] Phase 2: Data models and display
- [ ] Phase 3: Python MT5 integration
- [ ] Phase 4: Live updates with file watching
- [ ] Phase 5: Multi-tab interface
- [ ] Phase 6: Backtest triggering
- [ ] Phase 7: EA compilation support

## Contributing

This is a personal project, but suggestions and feedback are welcome!

## License

MIT

## Acknowledgments

- [Ratatui](https://github.com/ratatui/ratatui) - TUI framework
- [MetaTrader5](https://pypi.org/project/MetaTrader5/) - Python MT5 API
