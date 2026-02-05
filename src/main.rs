mod app;
mod service;
mod ui;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
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
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}
