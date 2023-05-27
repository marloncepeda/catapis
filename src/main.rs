use std::io;
use tui::{
    layout::{Constraint, Layout, Direction},
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};

use crossterm::{
    execute,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> Result<(), io::Error>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();


    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    loop{
        terminal.draw(|f| {
            let screen = Block::default()
                .style(Style::default().bg(Color::Reset))
                .borders(Borders::ALL);
            f.render_widget(screen, f.size());


            //obtener pantalla
            let _size = f.size();

            //UI
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ])
                .split(_size);

            let sidebar = Block::default()
                .title("Sidebar")
                .borders(Borders::ALL);
            f.render_widget(sidebar,chunks[0]);

            let content_lay = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(90),
                    Constraint::Percentage(10),
                ])
                .split(chunks[1]);

            let content = Block::default()
                .title("Content")
                .borders(Borders::ALL);
            f.render_widget(content,content_lay[0]);

            let footer = Block::default()
                .title("Footer")
                .borders(Borders::ALL);
            f.render_widget(footer,content_lay[1]);
            
        });

        //manejados de eventos con keyboard
        if let Event::Key(KeyEvent { code, ..}) = event::read()? {
            if code == KeyCode::Char('q'){
                break;
            }
        }
    }

    Ok(())
}