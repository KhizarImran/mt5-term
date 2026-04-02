mod app;
mod config;
mod models;
mod setup;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, path::Path, process::{Child, Command}};

use app::App;
use config::Config;

fn main() -> Result<()> {
    // Check for config, run setup if needed
    if !Config::exists() {
        println!("\n[!] No configuration found. Running first-time setup...\n");

        match setup::run_setup() {
            Ok(_) => {
                println!("\n[OK] Setup complete!\n");
            }
            Err(e) => {
                eprintln!("\nSetup failed: {}", e);
                eprintln!("\nYou can run setup again anytime by deleting config.json\n");
                std::process::exit(1);
            }
        }
    }

    // Load config (currently just validates it exists)
    let _config = Config::load()?;

    // Start Python service as subprocess
    println!("Starting MT5 service...");
    let python_service = start_python_service()?;

    println!("Waiting for data...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and load initial data
    let mut app = App::new();
    let data_path = Path::new("data/positions.json");
    if data_path.exists() {
        let _ = app.load_data(data_path);
    }

    // Run the app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    // Cleanup: kill Python service
    if let Some(mut child) = python_service {
        let _ = child.kill();
    }

    Ok(())
}

fn start_python_service() -> Result<Option<Child>> {
    let python_script = Path::new("python/mt5_service.py");

    if !python_script.exists() {
        eprintln!("Warning: python/mt5_service.py not found. Running without MT5 service.");
        eprintln!("The TUI will show mock data if available in data/positions.json");
        return Ok(None);
    }

    // Try to find Python executable
    let python_cmd = if cfg!(target_os = "windows") {
        "python"
    } else {
        "python3"
    };

    match Command::new(python_cmd)
        .arg(python_script)
        .spawn()
    {
        Ok(child) => {
            println!("[OK] MT5 service started (PID: {})", child.id());
            Ok(Some(child))
        }
        Err(e) => {
            eprintln!("Warning: Could not start Python service: {}", e);
            eprintln!("You can start it manually: python python/mt5_service.py");
            Ok(None)
        }
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(app, f))?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.quit();
                        }
                        KeyCode::Char('r') => {
                            // Reload data
                            let data_path = Path::new("data/positions.json");
                            let _ = app.load_data(data_path);
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
