"""
Configuration for MT5 Service.

You can customize these settings if needed, but the defaults should work
for most setups where MT5 is running locally.
"""

# MT5 Terminal Path (optional, auto-detects if not specified)
MT5_PATH = None  # e.g., r"C:\Program Files\MetaTrader 5\terminal64.exe"

# Account credentials (optional, uses current logged-in account if not specified)
MT5_LOGIN = None  # e.g., 12345678
MT5_PASSWORD = None  # e.g., "your_password"
MT5_SERVER = None  # e.g., "MetaQuotes-Demo"

# Data export settings
DATA_DIR = "../data"
UPDATE_INTERVAL_SECONDS = 5

# Logging
VERBOSE = True
