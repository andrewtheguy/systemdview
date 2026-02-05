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

    // Header / Search bar
    let header = if app.search_mode {
        let search_text = format!("/{}_", app.search_query);
        Paragraph::new(search_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Search"))
    } else if !app.search_query.is_empty() {
        let search_info = format!(
            "Search: {} ({} matches)",
            app.search_query,
            app.filtered_indices.len()
        );
        Paragraph::new(search_info)
            .style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::ALL))
    } else {
        Paragraph::new("SystemD Services")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL))
    };
    frame.render_widget(header, chunks[0]);

    // Services list
    if let Some(ref error) = app.error {
        let error_msg = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        frame.render_widget(error_msg, chunks[1]);
    } else {
        let items: Vec<ListItem> = app
            .filtered_indices
            .iter()
            .map(|&i| &app.services[i])
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

        let title = if app.search_query.is_empty() {
            format!("Services ({})", app.services.len())
        } else {
            format!(
                "Services ({}/{})",
                app.filtered_indices.len(),
                app.services.len()
            )
        };

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, chunks[1], &mut app.list_state);
    }

    // Footer with keybindings
    let footer_text = if app.search_mode {
        "Type to search | Esc/Enter: Exit search"
    } else if !app.search_query.is_empty() {
        "q: Quit | /: Search | Esc: Clear search | j/k: Navigate | r: Refresh"
    } else {
        "q/Esc: Quit | /: Search | j/↓: Down | k/↑: Up | g: Top | G: Bottom | r: Refresh"
    };
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}
