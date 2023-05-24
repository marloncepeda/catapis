use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Terminal,
};
use crossterm::{
    execute,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};

struct Plugin {
    content: ContentPlugin,
    sidebar: SidebarPlugin,
}
mod sidebar;
use crate::sidebar::SidebarPlugin;

struct ContentPlugin;

impl ContentPlugin {
    fn render(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let content = Paragraph::new("Content")
            .block(Block::default().borders(Borders::ALL).title("Content"));
        f.render_widget(content, area);
    }
}

struct Modal {
    is_open: bool,
    name: String,
    email: String,
    password: String,
}

impl Modal {
    fn new() -> Self {
        Self {
            is_open: false,
            name: String::new(),
            email: String::new(),
            password: String::new(),
        }
    }

    fn open(&mut self) {
        self.is_open = true;
    }

    fn close(&mut self) {
        self.is_open = false;
        self.name.clear();
        self.email.clear();
        self.password.clear();
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.is_open {
            match key_event.code {
                KeyCode::Esc => self.close(),
                KeyCode::Enter => {
                    // Lógica para procesar los datos del formulario
                    // Por ejemplo, podrías imprimirlos en la consola
                    println!("Name: {}", self.name);
                    println!("Email: {}", self.email);
                    println!("Password: {}", self.password);
                    self.close();
                }
                _ => {}
            }
        }
    }

    fn render(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        if self.is_open {
            let modal = Block::default()
                .title("Modal")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::White))
                .border_style(Style::default().fg(Color::Black));

            let text = vec![
                Spans::from(vec![Span::raw("Please fill out the form:")]),
                Spans::from(vec![Span::raw("")]),
            ];
            let text_widget = Paragraph::new(text)
                .style(Style::default().fg(Color::Black))
                .block(modal.clone())
                .alignment(tui::layout::Alignment::Center);
            f.render_widget(text_widget, area);

            let form = List::new(vec![
                ListItem::new(Spans::from(vec![
                    Span::raw("Name: "),
                    Span::styled(&self.name, Style::default().fg(Color::Blue)),
                ])),
                ListItem::new(Spans::from(vec![
                    Span::raw("Email: "),
                    Span::styled(&self.email, Style::default().fg(Color::Blue)),
                ])),
                ListItem::new(Spans::from(vec![
                    Span::raw("Password: "),
                    Span::styled(&self.password, Style::default().fg(Color::Blue)),
                ])),
            ])
            .block(modal)
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("> ");
            f.render_widget(form, area);
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Inicializar la terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, event::EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Estado del menú lateral
    let mut sidebar_visible = true;

    // Crear los plugins
    let plugin = Plugin {
        content: ContentPlugin,
        sidebar: SidebarPlugin,
    };

    // Crear el modal
    let mut modal = Modal::new();

    // Bucle principal de la interfaz de usuario
    loop {
        terminal.draw(|f| {
            // Borrar la pantalla
            let screen = Block::default()
                .style(Style::default().bg(Color::Reset))
                .borders(Borders::ALL);
            f.render_widget(screen, f.size());

            // Obtener el tamaño de la terminal
            let size = f.size();

            // Diseño de la interfaz
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(if sidebar_visible { 20 } else { 0 }),
                    Constraint::Percentage(if sidebar_visible { 80 } else { 100 }),
                ])
                .split(size);

            // Renderizar el menú lateral si está visible
            if sidebar_visible {
                plugin.sidebar.render(f, chunks[0]);
            }

            plugin.content.render(f, chunks[1]);

            // Renderizar el modal centrado en la página
            if modal.is_open {
                let modal_area = centered_rect(60, 20, f.size());
                modal.render(f, modal_area);
            }
        })?;

        // Manejo de eventos
        if let Event::Key(key_event) = event::read()? {
            if key_event.code == KeyCode::Char('q') {
                break;
            } else if key_event.code == KeyCode::Char('m') {
                modal.open();
            } else {
                modal.handle_key_event(key_event);
            }
        }
    }

    // Restaurar la pantalla original
    execute!(terminal.backend_mut(), event::DisableMouseCapture)?;
    disable_raw_mode()?;

    Ok(())
}

// Función de utilidad para centrar un rectángulo en otro rectángulo
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width - width) / 2;
    let y = area.y + (area.height - height) / 2;
    Rect::new(x, y, width, height)
}
