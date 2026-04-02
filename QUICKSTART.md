# Quick Start Guide

## The Simplest Way to Run MT5 Terminal TUI

### Prerequisites

1. **Rust** installed: https://rustup.rs/
2. **Python 3.8+** installed: https://www.python.org/
3. **MetaTrader 5** installed (optional for testing)

### Step 1: Install Dependencies

```bash
pip install -r python/requirements.txt
```

### Step 2: Run the Application

```bash
cargo run --release
```

**That's it!** 🎉

## What Happens on First Run

The application will automatically guide you through setup:

```
[!] No configuration found. Running first-time setup...

============================================================
MT5 Terminal - First Time Setup
============================================================

Step 1: MT5 Terminal Location
------------------------------------------------------------
[OK] Found MT5 at: C:\Program Files\MetaTrader 5\terminal64.exe
Use this path? (Y/n): [press Enter]

Step 2: MT5 Account Credentials
------------------------------------------------------------
Enter your MT5 account details.
(Leave blank to connect to currently logged-in account)

Account number: [press Enter to use current session]

Step 3: Data Settings
------------------------------------------------------------
Output directory [./data]: [press Enter]
Update interval in seconds [5]: [press Enter]

Step 4: Save Configuration
------------------------------------------------------------
Save this configuration? (Y/n): [press Enter]

[OK] Configuration saved!
```

After setup, the application will:
1. ✅ Start MT5 service automatically (background process)
2. ✅ Launch the TUI
3. ✅ Show your trades in real-time

## What You'll See

```
┌─ MT5 Terminal ──────────────────────────────────────────┐
│ ● Connected (5s ago) | Balance: $10000.00 | Equity: ... │
└─────────────────────────────────────────────────────────┘
┌─ Open Positions (3) ────────────────────────────────────┐
│ Ticket       Symbol  Type  Volume  Open     Current  ... │
│ 123456789    EURUSD  buy   0.10    1.08500  1.08750  ... │
│ ...                                                      │
└─────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────┐
│ q/ESC: Quit | r: Reload data                            │
└─────────────────────────────────────────────────────────┘
```

**Connection Indicator:**
- 🟢 **Green dot** = Data is fresh (updated < 10s ago)
- 🔴 **Red dot** = Data is stale (updated > 10s ago)

## Keyboard Controls

| Key | Action |
|-----|--------|
| `q` / `ESC` | Quit the application |
| `r` | Reload data manually |

## Configuration Modes

### Recommended: Use Current Session

When prompted for account credentials, **just press Enter**:
- Uses whichever MT5 account is currently logged in
- No password stored
- More secure

### Alternative: Auto-Login

Enter your credentials when prompted:
- Account number: `12345678`
- Password: `your_password`
- Server: `MetaQuotes-Demo`

The service will automatically log in to MT5.

## Testing Without MT5

Want to try it without connecting to MT5?

Create mock data:
```bash
python test_connection.py
cargo run --release
```

## Troubleshooting

### "Could not start Python service"

Make sure Python is in your PATH:
```bash
python --version
```

### "MT5 initialization failed"

1. Ensure MT5 terminal is **running**
2. Make sure you're **logged into an account**
3. Try running MT5 as **administrator**

### "No configuration found" (every time)

The setup failed. Check:
- You have write permissions in the directory
- You pressed 'Y' to save configuration

Delete `config.json` and try again.

## Advanced Usage

### Re-run Setup

Delete the config and restart:
```bash
rm config.json
cargo run
```

### Run Python Service Manually

If you prefer to run the Python service yourself:
```bash
# Terminal 1
python python/mt5_service.py

# Terminal 2
cargo run
```

### View Python Service Logs

The Python service runs in the background. To see its output:
```bash
python python/mt5_service.py
```

### Configuration File Location

After setup, your configuration is saved to:
```
./config.json
```

**Security:** This file may contain your MT5 password. It's already in `.gitignore`.

## What's Different from Before

### Old Way (2 commands):
```bash
# Terminal 1
python python/mt5_service.py

# Terminal 2
cargo run
```

### New Way (1 command):
```bash
cargo run
```

The Rust application now:
- ✅ Checks for configuration on startup
- ✅ Runs interactive setup if needed
- ✅ Automatically starts Python service
- ✅ Manages the Python subprocess lifecycle

## Summary

1. **First time:** Run `cargo run` → Follow setup wizard
2. **Every time after:** Just run `cargo run`
3. **That's it!** Everything else is automatic

Enjoy your MT5 Terminal TUI! 🚀
