use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    // Header
    let header = Paragraph::new("SystemD Services")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    // Services list
    if let Some(ref error) = app.error {
        let error_msg = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        frame.render_widget(error_msg, chunks[1]);
    } else {
        let items: Vec<ListItem> = app
            .services
            .iter()
            .map(|service| {
                let status_color = service.status_color();
                let line = Line::from(vec![
                    Span::styled(
                        format!("{:12}", service.status_display()),
                        Style::default().fg(status_color),
                    ),
                    Span::raw(" "),
                    Span::styled(&service.unit, Style::default().fg(Color::White)),
                    Span::raw(" - "),
                    Span::styled(&service.description, Style::default().fg(Color::DarkGray)),
                ]);
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Services ({})", app.services.len())),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, chunks[1], &mut app.list_state);
    }

    // Footer with keybindings
    let footer = Paragraph::new("q/Esc: Quit | j/↓: Down | k/↑: Up | g/Home: Top | G/End: Bottom | r: Refresh")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}
