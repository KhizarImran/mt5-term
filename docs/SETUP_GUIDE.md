# MT5 Terminal - Setup Guide

## What's New: Automatic Configuration

The MT5 service now includes an **interactive setup wizard** that runs automatically on first use.

## Quick Start

### 1. Install Dependencies

```bash
pip install -r python/requirements.txt
```

### 2. Run the Service (First Time)

```bash
python python/mt5_service.py
```

The setup wizard will start automatically and guide you through:

## Setup Wizard Steps

### Step 1: MT5 Terminal Location

The wizard will auto-detect your MT5 installation:

```
Step 1: MT5 Terminal Location
------------------------------------------------------------
[OK] Found MT5 at: C:\Program Files\MetaTrader 5\terminal64.exe
Use this path? (Y/n):
```

- Press **Enter** to use the detected path
- Type **n** to enter a custom path

**Can't find MT5?** 
- Check common locations:
  - `C:\Program Files\MetaTrader 5\terminal64.exe`
  - `C:\Program Files (x86)\MetaTrader 5\terminal64.exe`
- Press Enter to skip (will use auto-detection later)

---

### Step 2: MT5 Account Credentials

```
Step 2: MT5 Account Credentials
------------------------------------------------------------
Enter your MT5 account details.
(Leave blank to connect to currently logged-in account)

Account number: 12345678
Password: ********
Server (e.g., MetaQuotes-Demo): MetaQuotes-Demo
```

**Two Options:**

**Option A: Enter Credentials** (for auto-login)
- Account number: Your MT5 account ID
- Password: Your MT5 password
- Server: Your broker's server name

**Option B: Leave Blank** (use current session)
- Just press Enter when asked for account number
- Will connect to whichever account is currently logged in to MT5
- **Recommended** if you're already logged into MT5

---

### Step 3: Data Settings

```
Step 3: Data Settings
------------------------------------------------------------
Output directory [./data]:
Update interval in seconds [5]:
```

**Defaults are fine for most users:**
- Output directory: `./data` (where JSON files are saved)
- Update interval: `5` seconds (how often to fetch MT5 data)

---

### Step 4: Save Configuration

```
Step 4: Save Configuration
------------------------------------------------------------
Configuration will be saved to: C:\...\mt5-term\config.json

Configuration preview:
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

Save this configuration? (Y/n):
```

Press **Enter** to save.

---

## Configuration File

After setup, your configuration is saved to `config.json`:

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

**Security Note:** This file contains your credentials. It's already in `.gitignore` and won't be committed.

---

## After Setup

Once configured, the service runs normally:

```bash
# Terminal 1: Start MT5 service
python python/mt5_service.py

# Terminal 2: Launch TUI
cargo run --release
```

**Service Output:**

```
============================================================
MT5 Service - Data Exporter for TUI
============================================================

[OK] Connected to MT5 terminal
--> Using currently logged-in account
[OK] Account: 12345678
[OK] Server: MetaQuotes-Demo
[OK] Balance: $10000.00

[OK] Output file: C:\...\mt5-term\data\positions.json
[OK] Update interval: 5 seconds

Press Ctrl+C to stop

[0001] Updated: 3 positions | Profit: $+125.50 | Time: 14:30:15
[0002] Updated: 3 positions | Profit: $+130.25 | Time: 14:30:20
...
```

---

## Re-running Setup

To reconfigure:

```bash
python python/setup.py
```

Or just delete `config.json` and run the service again.

---

## Troubleshooting

### "ERROR: MT5 initialization failed"

**Solutions:**
1. Make sure MT5 terminal is **running**
2. If using auto-login, ensure you're **logged into an account** in MT5
3. Try running MT5 as **administrator**
4. Check the terminal path in `config.json` is correct

---

### "ERROR: Login failed"

If you provided credentials:
1. Check account number is correct
2. Verify password
3. Confirm server name (case-sensitive!)
4. Try using **Option B** (leave credentials blank, use current session)

---

### "Could not auto-detect MT5"

Manually enter the path:
- Default: `C:\Program Files\MetaTrader 5\terminal64.exe`
- Alternative: `C:\Program Files (x86)\MetaTrader 5\terminal64.exe`

Or check where you installed MT5:
```bash
# Search for terminal64.exe
dir "C:\Program Files\MetaTrader 5\terminal64.exe"
```

---

## Configuration Modes

### Mode 1: Auto-Login (Stored Credentials)

```json
{
  "mt5": {
    "terminal_path": "C:\\Program Files\\MetaTrader 5\\terminal64.exe",
    "account": 12345678,
    "password": "your_password",
    "server": "MetaQuotes-Demo"
  }
}
```

**Pros:** 
- Fully automated
- Service can restart MT5 connection
- No need to keep MT5 logged in

**Cons:**
- Password stored in plaintext (local file only)
- Less secure

---

### Mode 2: Current Session (Recommended)

```json
{
  "mt5": {
    "terminal_path": "C:\\Program Files\\MetaTrader 5\\terminal64.exe",
    "account": null,
    "password": null,
    "server": null
  }
}
```

**Pros:**
- No credentials stored
- More secure
- Simple setup

**Cons:**
- Must keep MT5 logged in
- Manual login to MT5 required

---

## Testing Without MT5

Want to test the TUI without MT5?

Create `data/positions.json` manually:

```bash
python -c "import json; from datetime import datetime; json.dump({'timestamp': datetime.utcnow().strftime('%Y-%m-%dT%H:%M:%SZ'), 'account': {'balance': 10000, 'equity': 10125, 'margin': 450, 'free_margin': 9675, 'profit': 125.5}, 'positions': [{'ticket': 123456, 'symbol': 'EURUSD', 'type': 'buy', 'volume': 0.1, 'open_price': 1.085, 'current_price': 1.0875, 'profit': 25, 'open_time': '2026-04-02T09:00:00Z'}]}, open('data/positions.json', 'w'), indent=2)"
```

Then run:

```bash
cargo run --release
```

You'll see mock data displayed!

---

## Files Created

After setup, you'll have:

```
mt5-term/
├── config.json           # Your configuration (auto-created)
├── data/
│   └── positions.json   # Live data from MT5 (auto-created)
└── ...
```

Both files are in `.gitignore` and won't be tracked by git.

---

## Next Steps

1. ✅ Setup complete
2. ✅ Service connected to MT5
3. 🚀 Launch the TUI: `cargo run --release`
4. 📊 Monitor your trades!

**Keyboard shortcuts:**
- `q` / `ESC` - Quit
- `r` - Reload data manually

**Coming soon:**
- Auto-refresh (no need to press 'r')
- Multi-tab interface
- Trade history

---

## Support

Issues with setup? Check:
- `config.json` - verify settings
- MT5 terminal is running
- You're logged into an MT5 account
- Python dependencies installed: `pip install -r python/requirements.txt`

Delete `config.json` and run `python python/mt5_service.py` again to start fresh.
