#!/usr/bin/env python3
"""
Test script to demonstrate the connection indicator.
Creates test data files with different timestamps.
"""

import json
from datetime import datetime, timedelta

# Base data template
data_template = {
    "timestamp": "",
    "account": {
        "balance": 10000.00,
        "equity": 10125.50,
        "margin": 450.00,
        "free_margin": 9675.50,
        "profit": 125.50
    },
    "positions": [
        {
            "ticket": 123456789,
            "symbol": "EURUSD",
            "type": "buy",
            "volume": 0.10,
            "open_price": 1.08500,
            "current_price": 1.08750,
            "profit": 25.00,
            "open_time": "2026-04-02T09:15:30Z"
        },
        {
            "ticket": 123456790,
            "symbol": "GBPUSD",
            "type": "sell",
            "volume": 0.05,
            "open_price": 1.27300,
            "current_price": 1.27100,
            "profit": 10.00,
            "open_time": "2026-04-02T10:30:15Z"
        },
        {
            "ticket": 123456791,
            "symbol": "USDJPY",
            "type": "buy",
            "volume": 0.20,
            "open_price": 150.250,
            "current_price": 150.700,
            "profit": 90.50,
            "open_time": "2026-04-02T11:45:00Z"
        }
    ]
}

print("Connection Indicator Test")
print("=" * 50)
print()

# Test 1: Fresh data (connected)
print("1. Creating FRESH data (should show GREEN Connected)...")
data = data_template.copy()
data["timestamp"] = datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
with open("data/positions.json", "w") as f:
    json.dump(data, f, indent=2)
print(f"   Timestamp: {data['timestamp']}")
print("   → Run: cargo run")
print()

# Test 2: Stale data (disconnected)
print("2. To test STALE data (should show RED Disconnected):")
print("   Run this command:")
print("   python -c \"import json; d=json.load(open('data/positions.json')); d['timestamp']='2026-04-02T13:00:00Z'; json.dump(d, open('data/positions.json', 'w'), indent=2)\"")
print()

print("✓ Test data created!")
print()
print("Features:")
print("  - GREEN dot = Connected (data < 10 seconds old)")
print("  - RED dot = Disconnected (data > 10 seconds old)")
print("  - Shows 'Xs ago' timestamp")
print("  - Press 'r' to reload and see status update")
