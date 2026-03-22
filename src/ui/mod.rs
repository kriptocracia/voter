pub mod election_detail;
pub mod election_list;
pub mod help;
pub mod password;
pub mod results;
pub mod settings;
pub mod vote;
pub mod welcome;
pub mod widgets;

use ratatui::Frame;

use crate::app::{App, Screen};

/// Render the current screen based on app state.
pub fn render(app: &App, frame: &mut Frame) {
    match &app.screen {
        Screen::Welcome => welcome::render(app, frame),
        Screen::PasswordPrompt => password::render(app, frame),
        Screen::ElectionList => election_list::render(app, frame),
        Screen::ElectionDetail { election_id } => {
            election_detail::render(app, frame, election_id);
        }
        Screen::Vote { election_id } => vote::render(app, frame, election_id),
        Screen::Results { election_id } => results::render(app, frame, election_id),
        Screen::Settings => settings::render(app, frame),
    }

    // Render help overlay
    if app.show_help {
        help::render_overlay(frame);
    }

    // Status bar at the bottom
    widgets::status_bar::render(app, frame);
}
