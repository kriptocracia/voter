use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

/// Render a confirmation dialog overlay.
///
/// `title` — dialog title (e.g., "Confirm Vote")
/// `message_lines` — body text lines
/// `confirm_selected` — whether "Confirm" button is focused (vs "Go Back")
pub fn render(frame: &mut Frame, title: &str, message_lines: &[String], confirm_selected: bool) {
    let area = centered_rect(50, 40, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).split(inner);

    // Message
    let lines: Vec<Line> = message_lines
        .iter()
        .map(|l| Line::from(l.as_str()))
        .collect();
    let message = Paragraph::new(lines);
    frame.render_widget(message, chunks[0]);

    // Buttons
    let confirm_style = if confirm_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    let back_style = if !confirm_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red)
    };

    let buttons = Paragraph::new(Line::from(vec![
        Span::raw("    "),
        Span::styled(" Confirm ", confirm_style),
        Span::raw("    "),
        Span::styled(" Go Back ", back_style),
    ]))
    .centered();
    frame.render_widget(buttons, chunks[1]);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical[1])[1]
}
