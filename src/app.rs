use ratatui::widgets::ListState;

use crate::service::{fetch_services, SystemdService};

pub struct App {
    pub services: Vec<SystemdService>,
    pub list_state: ListState,
    pub should_quit: bool,
    pub error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            services: Vec::new(),
            list_state: ListState::default(),
            should_quit: false,
            error: None,
        };
        app.load_services();
        app
    }

    pub fn load_services(&mut self) {
        match fetch_services() {
            Ok(services) => {
                self.services = services;
                self.error = None;
                if !self.services.is_empty() && self.list_state.selected().is_none() {
                    self.list_state.select(Some(0));
                }
            }
            Err(e) => {
                self.error = Some(e);
            }
        }
    }

    pub fn next(&mut self) {
        if self.services.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.services.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.services.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.services.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn go_to_top(&mut self) {
        if !self.services.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn go_to_bottom(&mut self) {
        if !self.services.is_empty() {
            self.list_state.select(Some(self.services.len() - 1));
        }
    }
}
