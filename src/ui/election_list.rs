use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::App;
use voter::nostr::events::ElectionStatus;

pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(frame.area());

    // Title
    let title = Paragraph::new(Line::from(vec![Span::styled(
        " Elections ",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]))
    .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, chunks[0]);

    // Election list
    let election_ids = app.sorted_election_ids();

    if election_ids.is_empty() {
        let empty = Paragraph::new("No elections found. Waiting for announcements...")
            .style(Style::default().fg(Color::DarkGray))
            .centered();
        frame.render_widget(empty, chunks[1]);
    } else {
        let items: Vec<ListItem> = election_ids
            .iter()
            .enumerate()
            .map(|(i, eid)| {
                let election = &app.elections[eid];
                let status_color = match election.status {
                    ElectionStatus::Open => Color::Green,
                    ElectionStatus::InProgress => Color::Yellow,
                    ElectionStatus::Finished => Color::Blue,
                    ElectionStatus::Cancelled => Color::Red,
                };

                let voted = if app.persistent_state.has_voted(eid) {
                    Span::styled(" [voted]", Style::default().fg(Color::Green))
                } else if app.persistent_state.is_registered(eid) {
                    Span::styled(" [registered]", Style::default().fg(Color::Cyan))
                } else {
                    Span::raw("")
                };

                let style = if i == app.election_list_index {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD | Modifier::REVERSED)
                } else {
                    Style::default()
                };

                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!(" [{}] ", election.status),
                        Style::default().fg(status_color),
                    ),
                    Span::styled(&election.name, style),
                    Span::raw(format!(
                        " ({} candidates, {})",
                        election.candidates.len(),
                        election.rules_id
                    )),
                    voted,
                ]))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Available Elections "),
        );
        frame.render_widget(list, chunks[1]);
    }

    // Key hints
    let hints = Paragraph::new(Line::from(vec![
        Span::styled("j/k", Style::default().fg(Color::Yellow)),
        Span::raw(" navigate  "),
        Span::styled("Enter", Style::default().fg(Color::Yellow)),
        Span::raw(" select  "),
        Span::styled("s", Style::default().fg(Color::Yellow)),
        Span::raw(" settings  "),
        Span::styled("?", Style::default().fg(Color::Yellow)),
        Span::raw(" help  "),
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::raw(" quit"),
    ]));
    frame.render_widget(hints, chunks[2]);
}
