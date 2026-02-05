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

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.should_quit = true;
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
