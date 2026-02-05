mod app;
mod service;
mod ui;

use std::io::{self, stdout};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseButton, MouseEvent, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, Terminal};

use app::App;

fn main() -> io::Result<()> {
    // Setup terminal with mouse capture
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
            // Help can be toggled from anywhere
            if key.code == KeyCode::Char('?') {
                app.toggle_help();
                continue;
            }

            // Close help with Esc or any key if help is shown
            if app.show_help {
                app.show_help = false;
                continue;
            }

            // Calculate visible lines for scrolling
            let visible_lines = ui::get_logs_visible_lines(&terminal.get_frame(), app.show_logs);
            let visible_services = ui::get_services_visible_lines(&terminal.get_frame(), app.show_logs);

            if app.search_mode {
                // Handle search mode input
                match key.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        app.search_mode = false;
                    }
                    KeyCode::Backspace => {
                        app.search_query.pop();
                        app.update_filter();
                    }
                    KeyCode::Down => {
                        app.next();
                    }
                    KeyCode::Up => {
                        app.previous();
                    }
                    KeyCode::PageUp => {
                        if app.show_logs {
                            app.scroll_logs_up(visible_lines);
                        } else {
                            app.page_up(visible_services);
                        }
                    }
                    KeyCode::PageDown => {
                        if app.show_logs {
                            app.scroll_logs_down(visible_lines, visible_lines);
                        } else {
                            app.page_down(visible_services);
                        }
                    }
                    KeyCode::Char(c) => {
                        app.search_query.push(c);
                        app.update_filter();
                    }
                    _ => {}
                }
            } else {
                // Normal mode input
                match key.code {
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('l') => {
                        app.toggle_logs();
                    }
                    KeyCode::Esc => {
                        if !app.search_query.is_empty() {
                            app.clear_search();
                        } else {
                            app.should_quit = true;
                        }
                    }
                    KeyCode::Char('/') => {
                        app.search_mode = true;
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.previous();
                    }
                    KeyCode::Char('g') | KeyCode::Home => {
                        app.go_to_top();
                    }
                    KeyCode::Char('G') | KeyCode::End => {
                        app.go_to_bottom();
                    }
                    KeyCode::Char('r') => {
                        app.load_services();
                    }
                    KeyCode::Char('s') => {
                        app.cycle_status_filter();
                    }
                    KeyCode::Char('S') => {
                        app.clear_status_filter();
                    }
                    KeyCode::PageUp => {
                        if app.show_logs {
                            app.scroll_logs_up(visible_lines);
                        } else {
                            app.page_up(visible_services);
                        }
                    }
                    KeyCode::PageDown => {
                        if app.show_logs {
                            app.scroll_logs_down(visible_lines, visible_lines);
                        } else {
                            app.page_down(visible_services);
                        }
                    }
                    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.scroll_logs_up(visible_lines / 2);
                    }
                    KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.scroll_logs_down(visible_lines / 2, visible_lines);
                    }
                    _ => {}
                }
            }
            }
            Event::Mouse(mouse) => {
                let size = terminal.size()?;
                let frame_rect = Rect::new(0, 0, size.width, size.height);
                handle_mouse_event(&mut app, mouse, frame_rect);
            }
            _ => {}
        }

        if app.should_quit {
            break;
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn handle_mouse_event(app: &mut App, mouse: MouseEvent, frame_size: Rect) {
    // Don't handle mouse events when help is shown
    if app.show_help {
        return;
    }

    let regions = ui::get_layout_regions(frame_size, app.show_logs);

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if mouse_in_rect(mouse, regions.services_list) {
                // Click-to-select: calculate which item was clicked
                // Subtract 1 for the border/title row
                let y_in_list = mouse.row.saturating_sub(regions.services_list.y + 1);
                let clicked_index = app.list_state.offset() + y_in_list as usize;
                if clicked_index < app.filtered_indices.len() {
                    app.list_state.select(Some(clicked_index));
                }
            }
        }
        MouseEventKind::ScrollUp => {
            if mouse_in_rect(mouse, regions.services_list) {
                app.previous();
            } else if let Some(logs) = regions.logs_panel
                && mouse_in_rect(mouse, logs)
            {
                app.scroll_logs_up(3);
            }
        }
        MouseEventKind::ScrollDown => {
            if mouse_in_rect(mouse, regions.services_list) {
                app.next();
            } else if let Some(logs) = regions.logs_panel
                && mouse_in_rect(mouse, logs)
            {
                let visible = logs.height.saturating_sub(2) as usize;
                app.scroll_logs_down(3, visible);
            }
        }
        _ => {}
    }
}

fn mouse_in_rect(mouse: MouseEvent, rect: Rect) -> bool {
    mouse.column >= rect.x
        && mouse.column < rect.x + rect.width
        && mouse.row >= rect.y
        && mouse.row < rect.y + rect.height
}
