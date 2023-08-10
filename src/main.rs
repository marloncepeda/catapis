use std::io;
use tui::{
    layout::{Constraint, Layout, Direction, Rect},
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, List},
    Terminal,
};

use crossterm::{
    execute,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};

mod ui {
    pub mod modal;
}
use ui::modal::{Modal, handle_event};



fn main() -> Result<(), io::Error>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let mut visible_sidebar = true;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut modal = ui::modal::Modal::new();

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
                    Constraint::Percentage(if visible_sidebar {20} else {0}),
                    Constraint::Percentage(if visible_sidebar {80} else {100}),
                ])
                .split(_size);

            let sidebar = Block::default()
                .title("Sidebar")
                .borders(Borders::ALL);

            if visible_sidebar {
                f.render_widget(sidebar,chunks[0]);
            }
            

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
    
            if modal.is_open {
                let modal_area = center_rect(60,20,_size);
                modal.render(f, modal_area);
            }
        });

        //manejados de eventos con keyboard
        
        if let Ok(event) = event::read() {
            handle_event(event, &mut modal);
        }
        

        if let Event::Key(KeyEvent { code, ..}) = event::read()? {
            if code == KeyCode::Char('q'){
                disable_raw_mode()?;
                break;
            }

        }
        if let Event::Key(KeyEvent { code, modifiers:event::KeyModifiers::CONTROL }) = event::read()? {
            if code == KeyCode::Char('j'){
                visible_sidebar = !visible_sidebar
            }
        }

    }

    Ok(())
}

fn center_rect(width:u16, height:u16, area:Rect) -> Rect{
    let x = area.x + (area.width - width)/2;
    let y = area.y + (area.height - height)/2;
    Rect::new(x,y,width,height)
}