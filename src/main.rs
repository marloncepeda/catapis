use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
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

struct ContentPlugin;

impl ContentPlugin {
    fn render(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: tui::layout::Rect) {
        let content = Paragraph::new("Content")
            .block(Block::default().borders(Borders::ALL).title("Content"));
        f.render_widget(content, area);
    }
}

struct SidebarPlugin;

impl SidebarPlugin {
    fn render(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: tui::layout::Rect) {
        let sidebar = Paragraph::new("Sidebar")
            .block(Block::default().borders(Borders::ALL).title("Sidebar"));
        f.render_widget(sidebar, area);
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
        })?;

        // Manejo de eventos
        if let Event::Key(KeyEvent { code, modifiers: event::KeyModifiers::CONTROL }) = event::read()? {
            if code == KeyCode::Char('j') {
                sidebar_visible = !sidebar_visible;
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            if code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // Restaurar la pantalla original
    execute!(terminal.backend_mut(), event::DisableMouseCapture)?;
    disable_raw_mode()?;

    Ok(())
}

