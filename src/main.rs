mod app;
mod config;
mod crypto;
mod error;
mod identity;
mod nostr;
mod state;
mod ui;

use std::io;

use crossterm::event::{Event, EventStream};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use futures::StreamExt;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tokio::sync::mpsc;

use app::{Action, App, Screen, ShouldQuit};
use config::AppConfig;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config_path = config::config_path();
    let config = AppConfig::load(&config_path)?;
    let persistent_state = AppState::load(&config.state_path())?;

    // Create action channel
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    // Determine initial screen
    let identity_path = &config.identity.path.clone();
    let initial_screen = if identity::identity_exists(identity_path) {
        if identity::identity_is_encrypted(identity_path) {
            Screen::PasswordPrompt
        } else {
            // Load identity directly
            Screen::ElectionList
        }
    } else {
        Screen::Welcome
    };

    let mut app = App::new(config, persistent_state, action_tx.clone());
    app.screen = initial_screen.clone();

    // If we can load identity directly, do it
    if matches!(initial_screen, Screen::ElectionList) {
        match identity::load_identity(None, identity_path) {
            Ok(keys) => app.keys = Some(keys),
            Err(_) => app.screen = Screen::Welcome,
        }
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stderr();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Spawn crossterm event reader
    let tx = action_tx.clone();
    tokio::spawn(async move {
        let mut reader = EventStream::new();
        while let Some(Ok(event)) = reader.next().await {
            match event {
                Event::Key(key) => {
                    let _ = tx.send(Action::KeyPress(key.code));
                }
                Event::Resize(_, _) => {
                    // Terminal will redraw on next tick
                }
                _ => {}
            }
        }
    });

    // Main loop
    loop {
        terminal.draw(|f| ui::render(&app, f))?;

        if let Some(action) = action_rx.recv().await
            && app.update(action) == ShouldQuit::Yes
        {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Save state
    let state_path = app.config.state_path();
    app.persistent_state.save(&state_path)?;

    Ok(())
}
