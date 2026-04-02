# mt5-term

A terminal user interface for MetaTrader 5, written in Rust.

Monitor your trades, account balance, and positions in real-time from your terminal.

## Features

- 📊 Real-time position monitoring
- 💰 Account balance and equity tracking  
- 🎨 Color-coded P&L (green for profit, red for loss)
- ⚡ Lightweight and fast
- ⌨️ Keyboard-driven interface

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Python 3.8+](https://www.python.org/downloads/)
- [MetaTrader 5](https://www.metatrader5.com/) terminal installed

### Setup

```bash
# Clone the repository
git clone <your-repo-url>
cd mt5-term

# Install Python dependencies
pip install -r python/requirements.txt

# Build the project
cargo build --release
```

## Usage

### Running the Application

Just run:

```bash
cargo run --release
```

That's it! On first run, the setup wizard will guide you through configuration:

**Setup Steps:**
1. 🔍 Auto-detect MT5 installation path
2. 💳 Enter MT5 account credentials (or use currently logged-in account)  
3. ⚙️ Configure data export settings
4. 💾 Save to `config.json`

The application will then:
- ✅ Automatically start the MT5 service
- ✅ Launch the TUI
- ✅ Display your trades in real-time

**No need to run Python separately!**

### Testing Without MT5

If MT5 is not available, you can test with mock data:

```bash
# Create mock data
python test_connection.py

# Run TUI
cargo run --release
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` / `ESC` | Quit |
| `r` | Reload data |

## Development

```bash
# Run in debug mode
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

### Architecture

```
MT5 Terminal → Python Service → JSON File → Rust TUI
```

The Python service polls MT5 every 5 seconds and writes position data to `data/positions.json`. The Rust TUI reads this file and displays it in a formatted table.

## Roadmap

- [x] Basic position display
- [x] Account info in header
- [x] Color-coded P&L
- [ ] Auto-refresh (file watching)
- [ ] Multi-tab interface
- [ ] Trade history view
- [ ] EA compilation support
- [ ] Backtest triggering

## License

MIT
