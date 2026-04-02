# CLAUDE.md

This file provides context for Claude Code when working on this project.

## Project Overview

**mt5-term** is a Terminal User Interface (TUI) for MetaTrader 5, written in Rust. It allows traders to monitor their MT5 positions, account balance, and P&L from the terminal.

### Architecture

```
MT5 Terminal (Windows)
    ↓ (MetaTrader5 Python API)
Python Service (mt5_service.py)
    ↓ (writes JSON every 5s)
data/positions.json
    ↓ (reads + displays)
Rust TUI (Ratatui)
```

**Why this architecture?**
- MT5 has no direct CLI or API for external programs
- Python's `MetaTrader5` library is the official integration method
- File-based IPC is simple, debuggable, and good enough for 5s refresh rate
- Keeps Rust TUI clean and focused on display logic

## Tech Stack

- **Rust**: TUI application using Ratatui + Crossterm
- **Python**: MT5 data exporter service
- **Build**: MinGW toolchain (GNU) instead of MSVC for simpler Windows builds

### Key Dependencies

```toml
ratatui = "0.28"      # TUI framework
crossterm = "0.28"    # Terminal backend
serde = "1.0"         # JSON serialization
chrono = "0.4"        # DateTime handling
tokio = "1"           # Async runtime (for future file watching)
notify = "6.1"        # File watching (not yet implemented)
```

## Project Structure

```
src/
├── main.rs           # Entry point, event loop, terminal setup
├── app.rs            # Application state management
├── ui.rs             # Ratatui rendering (tables, layout, colors)
└── models/
    ├── mod.rs        # Module exports
    ├── position.rs   # Position data structure
    ├── account.rs    # Account info structure
    └── data.rs       # Combined MT5Data structure

python/
├── mt5_service.py    # Main service: polls MT5, writes JSON
├── config.py         # Configuration (mostly unused, for future)
└── requirements.txt  # MetaTrader5>=5.0.4

data/
└── positions.json    # Shared data file (gitignored)
```

## Development Guidelines

### Building

```bash
# Debug build (18MB)
cargo build

# Release build (2.5MB, optimized)
cargo build --release

# Run
cargo run --release
```

### Testing

**With Mock Data** (no MT5 needed):
```bash
# Mock data already exists at data/positions.json
cargo run
```

**With Real MT5**:
```bash
# Terminal 1: Start Python service
python python/mt5_service.py

# Terminal 2: Run TUI
cargo run --release
```

### Code Style

- Keep it simple - favor clarity over cleverness
- No premature abstractions - we have <300 lines of Rust
- Color code profits: green = positive, red = negative
- Use `anyhow::Result` for error handling
- Format with `cargo fmt` before committing

## Common Tasks

### Adding a New Data Field

1. Update `data/positions.json` schema
2. Add field to struct in `src/models/*.rs` (with `#[serde]` attributes)
3. Update table rendering in `src/ui.rs` 
4. Update Python service in `python/mt5_service.py`

### Adding a New Keyboard Shortcut

Edit `src/main.rs`, in the `run_app()` function:

```rust
match key.code {
    KeyCode::Char('q') | KeyCode::Esc => app.quit(),
    KeyCode::Char('r') => { /* reload */ },
    KeyCode::Char('x') => { /* your new action */ },
    _ => {}
}
```

### Adding a New UI Widget

1. Create rendering function in `src/ui.rs`
2. Add to layout in `ui::render()` function
3. Update constraints if needed

## Current Status

### ✅ Completed (Phases 1-2)

- Basic TUI with header, content, footer
- Position table display with color-coded P&L
- Account info in header (balance, equity, profit)
- Keyboard controls (quit, reload)
- Python MT5 service with atomic JSON writes
- Mock data for testing

### 🚧 In Progress (Phase 3)

- File watching for auto-refresh (currently manual reload with 'r')
- Connection status indicator
- Better error handling for missing/invalid JSON

### 📋 Planned (Phases 4-7)

- Multi-tab interface (Positions, Account, History, Logs)
- Scrollable position list
- Backtest triggering via MetaEditor CLI
- EA compilation support
- Trade history view
- Performance metrics

## Important Notes

### Windows-Specific Setup

This project uses **MinGW (GNU) toolchain** instead of MSVC:
- Configured in `.cargo/config.toml`
- Avoids Visual Studio dependencies
- If you see linker errors, ensure MinGW is installed: `choco install mingw`

### Data Refresh Rate

Currently **5 seconds** (hardcoded in Python service). This is:
- Good enough for position monitoring
- Not too aggressive on MT5 API
- Can be made configurable later if needed

### JSON Schema

The `positions.json` format:

```json
{
  "timestamp": "ISO 8601 datetime",
  "account": {
    "balance": float,
    "equity": float,
    "margin": float,
    "free_margin": float,
    "profit": float
  },
  "positions": [
    {
      "ticket": u64,
      "symbol": string,
      "type": "buy" | "sell",
      "volume": float,
      "open_price": float,
      "current_price": float,
      "profit": float,
      "open_time": "ISO 8601 datetime"
    }
  ]
}
```

## Debugging

### Python Service Not Connecting

1. Check MT5 is running: Task Manager → terminal64.exe
2. Check you're logged into an account
3. Run Python service manually to see errors:
   ```bash
   python python/mt5_service.py
   ```

### TUI Shows "No Data"

1. Check `data/positions.json` exists and is valid JSON
2. Press `r` to reload manually
3. Check file timestamps: `ls -lh data/positions.json`

### Build Errors

- **"link.exe failed"**: MinGW not in PATH or not installed
- **"unused import"**: Clean up imports or use `#[allow(unused)]`
- **Linking takes forever**: This is normal for first build (~3 minutes)

## Future Enhancements

### Phase 3: Auto-Refresh

Use the `notify` crate to watch `data/positions.json`:
```rust
// In main.rs, spawn a tokio task
let (tx, rx) = channel();
let watcher = notify::recommended_watcher(tx)?;
watcher.watch("data/positions.json", RecursiveMode::NonRecursive)?;

// In event loop, check rx for file change events
```

### Phase 4: Multi-Tab Interface

Add `enum Tab { Positions, Account, History }` to `App`:
```rust
pub struct App {
    pub data: Option<Mt5Data>,
    pub current_tab: Tab,  // Add this
    // ...
}
```

Update UI to render different content based on `app.current_tab`.

### Phase 5: EA Compilation

Call MetaEditor CLI from Python service:
```python
import subprocess
subprocess.run([
    "C:/Program Files/MetaTrader 5/metaeditor64.exe",
    "/compile:path/to/EA.mq5",
    "/log:compile.log"
])
```

Parse log file and write results to JSON.

## Useful Commands

```bash
# Check file sizes
ls -lh target/x86_64-pc-windows-gnu/release/

# Watch JSON file updates
watch -n 1 'cat data/positions.json | jq .timestamp'

# Count lines of code
find src -name "*.rs" | xargs wc -l

# Clean build artifacts
cargo clean

# Update dependencies
cargo update
```

## Contact & Context

- **User**: Building this as a personal trading tool
- **Goal**: Simple, reliable MT5 monitoring from terminal
- **Constraints**: Windows-only (MT5 limitation), start simple
- **Philosophy**: MVP first, enhance later

When working on this project:
1. Prioritize working code over perfect code
2. Keep the architecture simple (file-based IPC is fine)
3. Test with mock data frequently
4. Don't over-engineer - we're at ~300 lines total
5. Real-time updates aren't critical (5s is acceptable)
