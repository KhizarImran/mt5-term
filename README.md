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
- [MetaTrader 5](https://www.metatrader5.com/) terminal (running and logged in)

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

### Running the TUI

**Step 1:** Start the MT5 data service (in one terminal):

```bash
python python/mt5_service.py
```

**Step 2:** Launch the TUI (in another terminal):

```bash
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
