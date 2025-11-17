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

    // Run the app
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    let mut render_interval = interval(Duration::from_millis(16)); // ~60 FPS

    loop {
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
