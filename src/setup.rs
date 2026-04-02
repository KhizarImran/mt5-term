use anyhow::Result;
use std::io::{self, Write};

use crate::config::{Config, DataConfig, Mt5Config};

pub fn run_setup() -> Result<Config> {
    println!("\n{}", "=".repeat(60));
    println!("MT5 Terminal - First Time Setup");
    println!("{}\n", "=".repeat(60));

    let mut config = Config::default();

    // Step 1: MT5 Terminal Path
    println!("Step 1: MT5 Terminal Location");
    println!("{}", "-".repeat(60));

    if let Some(detected_path) = Config::find_mt5_terminal() {
        println!("[OK] Found MT5 at: {}", detected_path);
        print!("Use this path? (Y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().is_empty() || input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
            config.mt5.terminal_path = Some(detected_path);
        } else {
            print!("Enter MT5 terminal path: ");
            io::stdout().flush()?;
            let mut path = String::new();
            io::stdin().read_line(&mut path)?;
            config.mt5.terminal_path = Some(path.trim().to_string());
        }
    } else {
        println!("[WARN] Could not auto-detect MT5 installation");
        print!("Enter MT5 terminal path (or press Enter to skip): ");
        io::stdout().flush()?;

        let mut path = String::new();
        io::stdin().read_line(&mut path)?;

        if !path.trim().is_empty() {
            config.mt5.terminal_path = Some(path.trim().to_string());
        }
    }

    println!();

    // Step 2: MT5 Account Credentials
    println!("Step 2: MT5 Account Credentials");
    println!("{}", "-".repeat(60));
    println!("Enter your MT5 account details.");
    println!("(Leave blank to connect to currently logged-in account)");
    println!();

    print!("Account number: ");
    io::stdout().flush()?;
    let mut account = String::new();
    io::stdin().read_line(&mut account)?;

    if !account.trim().is_empty() {
        config.mt5.account = account.trim().parse().ok();

        print!("Password: ");
        io::stdout().flush()?;
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;
        config.mt5.password = Some(password.trim().to_string());

        print!("Server (e.g., MetaQuotes-Demo): ");
        io::stdout().flush()?;
        let mut server = String::new();
        io::stdin().read_line(&mut server)?;
        config.mt5.server = Some(server.trim().to_string());
    } else {
        println!("--> Will use currently logged-in MT5 account");
    }

    println!();

    // Step 3: Data Settings
    println!("Step 3: Data Settings");
    println!("{}", "-".repeat(60));

    print!("Output directory [{}]: ", config.data.output_dir);
    io::stdout().flush()?;
    let mut output_dir = String::new();
    io::stdin().read_line(&mut output_dir)?;

    if !output_dir.trim().is_empty() {
        config.data.output_dir = output_dir.trim().to_string();
    }

    print!("Update interval in seconds [{}]: ", config.data.update_interval_seconds);
    io::stdout().flush()?;
    let mut interval = String::new();
    io::stdin().read_line(&mut interval)?;

    if !interval.trim().is_empty() {
        if let Ok(val) = interval.trim().parse() {
            config.data.update_interval_seconds = val;
        }
    }

    println!();

    // Step 4: Save Configuration
    println!("Step 4: Save Configuration");
    println!("{}", "-".repeat(60));
    println!("Configuration will be saved to: config.json");
    println!();
    println!("Configuration preview:");
    println!("{}", serde_json::to_string_pretty(&config)?);
    println!();

    print!("Save this configuration? (Y/n): ");
    io::stdout().flush()?;
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;

    if confirm.trim().is_empty() || confirm.trim().to_lowercase() == "y" || confirm.trim().to_lowercase() == "yes" {
        config.save()?;

        // Ensure data directory exists
        std::fs::create_dir_all(&config.data.output_dir)?;

        println!();
        println!("[OK] Configuration saved!");
        println!();

        Ok(config)
    } else {
        anyhow::bail!("Setup cancelled by user")
    }
}
