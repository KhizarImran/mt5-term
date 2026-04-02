# Changelog

## [Unreleased] - 2026-04-02

### Added ✨

- **One-Command Launch**: Just run `cargo run` - no separate Python service needed!
- **Automatic Setup Wizard**: First-run configuration built into Rust
  - Auto-detects MT5 installation path
  - Prompts for account credentials or uses current session
  - Saves to `config.json`
- **Connection Status Indicator**: Real-time status in header
  - 🟢 Green dot = Connected (data < 10 seconds old)
  - 🔴 Red dot = Disconnected (data > 10 seconds old)
  - Shows "Xs ago" timestamp
- **Subprocess Management**: Python service auto-starts and stops with TUI
- **Config Module**: Structured configuration with `config.rs`
- **Setup Module**: Interactive onboarding in `setup.rs`

### Changed 🔄

- **Simplified Workflow**: No longer need to run Python service separately
- **Python Service**: Now runs as background process spawned by Rust
- **Main Entry Point**: Setup logic integrated into `main.rs`

### Technical Details 🔧

**New Files:**
- `src/config.rs` - Configuration loading/saving with serde
- `src/setup.rs` - Interactive setup wizard (CLI prompts)
- `config.json` - User configuration (gitignored)

**Modified Files:**
- `src/main.rs` - Added setup check, subprocess spawning
- `src/app.rs` - Connection status logic
- `src/ui.rs` - Status indicator rendering
- `README.md` - Updated usage instructions
- `.gitignore` - Added config.json

**Architecture:**
```
cargo run
  ├─> Check config.json
  ├─> Run setup if needed (CLI prompts)
  ├─> Spawn python/mt5_service.py as subprocess
  ├─> Launch TUI (ratatui)
  └─> Kill Python on exit
```

### User Experience 🎨

**Before:**
```bash
# Terminal 1
python python/mt5_service.py

# Terminal 2  
cargo run
```

**After:**
```bash
cargo run
# That's it!
```

**First Run:**
```
[!] No configuration found. Running first-time setup...

============================================================
MT5 Terminal - First Time Setup
============================================================

Step 1: MT5 Terminal Location
------------------------------------------------------------
[OK] Found MT5 at: C:\Program Files\MetaTrader 5\terminal64.exe
Use this path? (Y/n): 
...
```

### Configuration Format 📝

```json
{
  "mt5": {
    "terminal_path": "C:\\Program Files\\MetaTrader 5\\terminal64.exe",
    "account": null,
    "password": null,
    "server": null
  },
  "data": {
    "output_dir": "./data",
    "update_interval_seconds": 5
  }
}
```

**Two modes:**
- **Mode 1**: Provide credentials (account, password, server) - auto-login
- **Mode 2**: Leave blank (null) - use currently logged-in account

### Breaking Changes ⚠️

None! The Python service still works standalone if you want to run it manually.

### Migration Guide 📦

If you have an existing installation:

**Nothing to do!** The new system is backwards compatible.

If you want to use the new one-command flow:
1. Delete any existing config files
2. Run `cargo run`
3. Follow the setup wizard

### Known Issues 🐛

- Python service output not visible in terminal (runs in background)
  - To see Python logs, run manually: `python python/mt5_service.py`
- On Windows, Python process may not always terminate cleanly
  - Check Task Manager if you see orphaned python.exe processes

### What's Next 🚀

- [ ] File watching for auto-refresh
- [ ] Multi-tab interface (Positions, Account, Logs)
- [ ] Show Python service logs in TUI
- [ ] Health check for Python service
- [ ] Restart Python service if it crashes
