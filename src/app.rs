use ratatui::widgets::ListState;

use crate::service::{fetch_services, SystemdService};

pub struct App {
    pub services: Vec<SystemdService>,
    pub list_state: ListState,
    pub should_quit: bool,
    pub error: Option<String>,
    pub search_query: String,
    pub search_mode: bool,
    pub filtered_indices: Vec<usize>,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            services: Vec::new(),
            list_state: ListState::default(),
            should_quit: false,
            error: None,
            search_query: String::new(),
            search_mode: false,
            filtered_indices: Vec::new(),
        };
        app.load_services();
        app
    }

    pub fn load_services(&mut self) {
        match fetch_services() {
            Ok(services) => {
                self.services = services;
                self.error = None;
                self.update_filter();
                if !self.filtered_indices.is_empty() && self.list_state.selected().is_none() {
                    self.list_state.select(Some(0));
                }
            }
            Err(e) => {
                self.error = Some(e);
            }
        }
    }

    pub fn update_filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_indices = (0..self.services.len()).collect();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_indices = self
                .services
                .iter()
                .enumerate()
                .filter(|(_, service)| {
                    service.unit.to_lowercase().contains(&query)
                        || service.description.to_lowercase().contains(&query)
                })
                .map(|(i, _)| i)
                .collect();
        }
        // Reset selection if current selection is out of bounds
        if let Some(selected) = self.list_state.selected() {
            if selected >= self.filtered_indices.len() {
                if self.filtered_indices.is_empty() {
                    self.list_state.select(None);
                } else {
                    self.list_state.select(Some(0));
                }
            }
        } else if !self.filtered_indices.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.update_filter();
    }

    pub fn next(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.filtered_indices.len() - 1 {
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
        if self.filtered_indices.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_indices.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn go_to_top(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn go_to_bottom(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.list_state.select(Some(self.filtered_indices.len() - 1));
        }
    }
}
