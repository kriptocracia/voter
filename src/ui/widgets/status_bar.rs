use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::{App, Screen};

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();
    // Status bar occupies the very last line
    let bar_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };

    let screen_name = match &app.screen {
        Screen::Welcome => "Setup",
        Screen::PasswordPrompt => "Unlock",
        Screen::ElectionList => "Elections",
        Screen::ElectionDetail { .. } => "Detail",
        Screen::Vote { .. } => "Vote",
        Screen::Results { .. } => "Results",
        Screen::Settings => "Settings",
    };

    let connection_color = if app.connected {
        Color::Green
    } else {
        Color::Red
    };
    let connection_text = if app.connected { "●" } else { "○" };

    let chunks = Layout::horizontal([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(20),
    ])
    .split(bar_area);

    // Connection indicator
    let conn = Paragraph::new(Span::styled(
        connection_text,
        Style::default().fg(connection_color),
    ));
    frame.render_widget(conn, chunks[0]);

    // Status message or screen name
    let status_text = app.status_message.as_deref().unwrap_or(screen_name);
    let status = Paragraph::new(Span::styled(
        status_text,
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(status, chunks[1]);

    // Screen name on the right
    let right = Paragraph::new(Line::from(vec![Span::styled(
        screen_name,
        Style::default().fg(Color::DarkGray),
    )]))
    .right_aligned();
    frame.render_widget(right, chunks[2]);
}
