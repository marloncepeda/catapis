use std::io;
use tui::{
    layout::{Constraint, Layout, Direction, Rect},
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, List},
    Terminal,
};


pub struct Modal {
    pub is_open: bool,
}

impl Modal {
    pub fn new() -> Self {
        Self {
            is_open: false,
        }
    } 

    pub fn render(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        if self.is_open {
            let modal = Block::default()
                .title("Modal")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Black))
                .style(Style::default().bg(Color::White));

            let form = List::new(vec![]).block(modal);

            f.render_widget(form, area);
        }
    }
}
