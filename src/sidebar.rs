use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use tui::backend::CrosstermBackend;


use std::io;

pub struct SidebarPlugin;

impl SidebarPlugin {
    pub fn render(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let sidebar = Paragraph::new("Sidebar")
            .block(Block::default().borders(Borders::ALL).title("Sidebar"));
        f.render_widget(sidebar, area);
    }
}
