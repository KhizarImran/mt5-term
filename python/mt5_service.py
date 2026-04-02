#!/usr/bin/env python3
"""
MT5 Service - Exports MetaTrader 5 data to JSON for the Rust TUI.

This service connects to a running MT5 terminal, fetches account info
and open positions, and writes them to a JSON file every 5 seconds.
"""

import MetaTrader5 as mt5
import json
import time
from datetime import datetime
from pathlib import Path
import sys
import subprocess


def load_config():
    """Load configuration from config.json."""
    config_path = Path(__file__).parent.parent / "config.json"

    if not config_path.exists():
        return None

    try:
        with open(config_path, 'r') as f:
            return json.load(f)
    except Exception as e:
        print(f"ERROR: Could not load config: {e}")
        return None


def run_setup():
    """Run interactive setup if config doesn't exist."""
    print()
    print("=" * 60)
    print("No configuration found!")
    print("=" * 60)
    print()
    print("Let's set up your MT5 connection.")
    print()

    setup_script = Path(__file__).parent / "setup.py"

    try:
        result = subprocess.run([sys.executable, str(setup_script)])
        if result.returncode == 0:
            return load_config()
        else:
            return None
    except Exception as e:
        print(f"ERROR: Setup failed: {e}")
        return None


def initialize_mt5(config):
    """Initialize connection to MT5 terminal."""
    mt5_config = config.get("mt5", {})

    # Initialize with path if provided
    terminal_path = mt5_config.get("terminal_path")
    if terminal_path:
        if not mt5.initialize(path=terminal_path):
            print(f"ERROR: Could not initialize MT5 at {terminal_path}")
            print("Error code:", mt5.last_error())
            return False
    else:
        if not mt5.initialize():
            print("ERROR: MT5 initialization failed")
            print("Error code:", mt5.last_error())
            return False

    print("✓ Connected to MT5 terminal")

    # Login with credentials if provided
    account = mt5_config.get("account")
    password = mt5_config.get("password")
    server = mt5_config.get("server")

    if account and password and server:
        print(f"Logging in to account {account} on {server}...")
        if not mt5.login(account, password=password, server=server):
            print("ERROR: Login failed")
            print("Error code:", mt5.last_error())
            return False
        print("✓ Login successful")
    else:
        print("→ Using currently logged-in account")

    # Display account info
    account_info = mt5.account_info()
    if account_info:
        print(f"✓ Account: {account_info.login}")
        print(f"✓ Server: {account_info.server}")
        print(f"✓ Balance: ${account_info.balance:.2f}")
    else:
        print("WARNING: Could not fetch account info")

    return True


def fetch_mt5_data():
    """Fetch current positions and account info from MT5."""
    try:
        # Get account info
        account = mt5.account_info()
        if account is None:
            print("WARNING: Could not fetch account info")
            return None

        # Get open positions
        positions = mt5.positions_get()
        if positions is None:
            print("WARNING: Could not fetch positions")
            positions = ()

        # Build data structure
        data = {
            "timestamp": datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ"),
            "account": {
                "balance": float(account.balance),
                "equity": float(account.equity),
                "margin": float(account.margin),
                "free_margin": float(account.margin_free),
                "profit": float(account.profit),
            },
            "positions": []
        }

        # Add positions
        for pos in positions:
            position_data = {
                "ticket": int(pos.ticket),
                "symbol": pos.symbol,
                "type": "buy" if pos.type == mt5.ORDER_TYPE_BUY else "sell",
                "volume": float(pos.volume),
                "open_price": float(pos.price_open),
                "current_price": float(pos.price_current),
                "profit": float(pos.profit),
                "open_time": datetime.fromtimestamp(pos.time).strftime("%Y-%m-%dT%H:%M:%SZ"),
            }
            data["positions"].append(position_data)

        return data

    except Exception as e:
        print(f"ERROR fetching MT5 data: {e}")
        return None


def write_json_atomic(data, filepath):
    """Write JSON data atomically to avoid partial reads."""
    temp_path = Path(str(filepath) + ".tmp")
    final_path = Path(filepath)

    try:
        # Write to temporary file
        with open(temp_path, 'w') as f:
            json.dump(data, f, indent=2)

        # Atomic rename (replaces existing file)
        temp_path.replace(final_path)
        return True

    except Exception as e:
        print(f"ERROR writing JSON: {e}")
        if temp_path.exists():
            temp_path.unlink()
        return False


def main():
    """Main service loop."""
    print("=" * 60)
    print("MT5 Service - Data Exporter for TUI")
    print("=" * 60)
    print()

    # Load or create configuration
    config = load_config()

    if config is None:
        print("Running first-time setup...")
        config = run_setup()

        if config is None:
            print("ERROR: Setup failed or was cancelled")
            sys.exit(1)

    # Initialize MT5 with configuration
    if not initialize_mt5(config):
        print("\nERROR: Could not connect to MT5")
        print("\nTroubleshooting:")
        print("1. Ensure MT5 terminal is running")
        print("2. Check that you're logged into an MT5 account")
        print("3. Try running MT5 as administrator")
        print("4. Re-run setup: python python/setup.py")
        sys.exit(1)

    # Setup paths from config
    data_config = config.get("data", {})
    data_dir = Path(__file__).parent.parent / data_config.get("output_dir", "data")
    data_dir.mkdir(parents=True, exist_ok=True)
    output_file = data_dir / "positions.json"

    update_interval = data_config.get("update_interval_seconds", 5)

    print(f"\n✓ Output file: {output_file}")
    print(f"✓ Update interval: {update_interval} seconds")
    print("\nPress Ctrl+C to stop\n")

    update_count = 0

    try:
        while True:
            # Fetch data from MT5
            data = fetch_mt5_data()

            if data:
                # Write to JSON
                if write_json_atomic(data, output_file):
                    update_count += 1
                    pos_count = len(data["positions"])
                    profit = data["account"]["profit"]

                    print(f"[{update_count:04d}] Updated: {pos_count} positions | "
                          f"Profit: ${profit:+.2f} | "
                          f"Time: {datetime.now().strftime('%H:%M:%S')}")
                else:
                    print(f"[{update_count:04d}] ERROR: Failed to write JSON")
            else:
                print(f"[{update_count:04d}] ERROR: Failed to fetch MT5 data")

            # Wait before next update
            time.sleep(update_interval)

    except KeyboardInterrupt:
        print("\n\n✓ Service stopped by user")

    except Exception as e:
        print(f"\n\nERROR: {e}")
        import traceback
        traceback.print_exc()

    finally:
        # Cleanup
        mt5.shutdown()
        print("✓ Disconnected from MT5")


if __name__ == "__main__":
    main()
