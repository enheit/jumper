mod app;
mod clipboard;
mod config;
mod events;
mod file_ops;
mod fuzzy;
mod ui;

use anyhow::Result;
use app::App;
use config::Config;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use directories::BaseDirs;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load().unwrap_or_else(|_| {
        eprintln!("Warning: Could not load config, using defaults");
        Config::default()
    });

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(config)?;

    // Start directory size calculations
    app.start_dir_size_calculation();

    // Run the app
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    // Write current directory to temp file for shell integration
    if let Some(base_dirs) = BaseDirs::new() {
        let jumper_cache = base_dirs.cache_dir().join("jumper");
        if let Err(e) = std::fs::create_dir_all(&jumper_cache) {
            eprintln!("Warning: Could not create cache directory: {}", e);
        } else {
            let last_dir_file = jumper_cache.join("lastdir");
            if let Err(e) = std::fs::write(&last_dir_file, app.current_dir.to_string_lossy().as_bytes()) {
                eprintln!("Warning: Could not write lastdir file: {}", e);
            }
        }
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    let mut render_interval = interval(Duration::from_millis(16)); // ~60 FPS
    let mut flash_timer: Option<tokio::time::Instant> = None;
    let mut error_timer: Option<tokio::time::Instant> = None;

    loop {
        // Clear flash copied paths after timeout
        if let Some(timer) = flash_timer {
            if timer.elapsed() >= Duration::from_millis(app.config.behavior.flash_duration_ms) {
                app.flash_copied_paths.clear();
                flash_timer = None;
            }
        }

        // Clear error message after 3 seconds
        if let Some(timer) = error_timer {
            if timer.elapsed() >= Duration::from_millis(3000) {
                app.error_message = None;
                error_timer = None;
            }
        }

        // Set timer when flash copied paths is shown
        if !app.flash_copied_paths.is_empty() && flash_timer.is_none() {
            flash_timer = Some(tokio::time::Instant::now());
        }

        // Set timer when error message is shown
        if app.error_message.is_some() && error_timer.is_none() {
            error_timer = Some(tokio::time::Instant::now());
        } else if app.error_message.is_none() {
            error_timer = None;
        }

        // Check for directory size updates
        app.check_dir_size_updates();

        // Draw UI
        terminal.draw(|f| ui::render_ui(f, app))?;

        // Handle events
        tokio::select! {
            _ = render_interval.tick() => {
                // Render tick - UI already drawn above
            }
            _ = tokio::time::sleep(Duration::from_millis(10)) => {
                if event::poll(Duration::from_millis(0))? {
                    if let Event::Key(key) = event::read()? {
                        events::handle_key_event(app, key).await?;
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
