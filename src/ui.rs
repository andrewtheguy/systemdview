use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &mut App) {
    // Load logs for selected service if selection changed (only if logs are visible)
    if app.show_logs {
        app.load_logs_for_selected();
    }

    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    // Conditionally split middle section for logs panel
    let (services_area, logs_area) = if app.show_logs {
        let middle_chunks = Layout::horizontal([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(chunks[1]);
        (middle_chunks[0], Some(middle_chunks[1]))
    } else {
        (chunks[1], None)
    };

    // Header / Search bar
    let header = if app.search_mode {
        let search_text = format!("/{}_", app.search_query);
        Paragraph::new(search_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Search"))
    } else if !app.search_query.is_empty() || app.status_filter.is_some() {
        let mut info_parts = Vec::new();
        if !app.search_query.is_empty() {
            info_parts.push(format!("Search: {}", app.search_query));
        }
        if let Some(ref status) = app.status_filter {
            info_parts.push(format!("Status: {}", status));
        }
        let info = format!("{} ({} matches)", info_parts.join(" | "), app.filtered_indices.len());
        Paragraph::new(info)
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
        frame.render_widget(error_msg, services_area);
    } else {
        let items: Vec<ListItem> = app
            .filtered_indices
            .iter()
            .map(|&i| &app.services[i])
            .map(|service| {
                let status_color = service.status_color();
                let line = Line::from(vec![
                    Span::styled(
                        format!("{:8}", service.status_display()),
                        Style::default().fg(status_color),
                    ),
                    Span::styled(&service.unit, Style::default().fg(Color::White)),
                ]);
                ListItem::new(line)
            })
            .collect();

        let title = if app.search_query.is_empty() && app.status_filter.is_none() {
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

        frame.render_stateful_widget(list, services_area, &mut app.list_state);
    }

    // Logs panel (only if visible)
    if let Some(logs_area) = logs_area {
        let logs_title = if let Some(ref service_name) = app.last_selected_service {
            format!("Logs: {}", service_name)
        } else {
            "Logs".to_string()
        };

        // Calculate visible area (subtract 2 for borders)
        let visible_lines = logs_area.height.saturating_sub(2) as usize;

        // Create log content with scroll
        let log_lines: Vec<Line> = app
            .logs
            .iter()
            .skip(app.logs_scroll)
            .take(visible_lines)
            .map(|line| Line::from(line.as_str()))
            .collect();

        let scroll_info = if !app.logs.is_empty() {
            format!(
                " [{}-{}/{}]",
                app.logs_scroll + 1,
                (app.logs_scroll + visible_lines).min(app.logs.len()),
                app.logs.len()
            )
        } else {
            String::new()
        };

        let logs_paragraph = Paragraph::new(log_lines)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{}{}", logs_title, scroll_info)),
            )
            .wrap(Wrap { trim: false });

        frame.render_widget(logs_paragraph, logs_area);
    }

    // Footer with keybindings
    let footer_text = if app.search_mode {
        "Type to search | Esc/Enter: Exit search"
    } else if !app.search_query.is_empty() || app.status_filter.is_some() {
        "q: Quit | /: Search | s: Status | l: Logs | Esc: Clear | j/k: Nav | r: Refresh"
    } else {
        "q/Esc: Quit | /: Search | s: Status | l: Logs | j/k: Nav | g/G: Top/Bottom | r: Refresh"
    };
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}

/// Returns the number of visible lines in the logs panel
pub fn get_logs_visible_lines(frame: &Frame, show_logs: bool) -> usize {
    if !show_logs {
        return 0;
    }

    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let middle_chunks = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(60),
    ])
    .split(chunks[1]);

    middle_chunks[1].height.saturating_sub(2) as usize
}
