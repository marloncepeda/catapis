use std::io;
use crossterm::event::{self, Event, KeyCode,KeyEvent};
use super::Modal;

pub fn handle_event(event: Event, modal: &mut Modal) {
    if let Event::Key(KeyEvent { code, .. }) = event {
        match code {
            KeyCode::Char('m') => {
                modal.is_open = !modal.is_open;
            }
            _ => {
                // Otros manejadores de eventos según corresponda
            }
        }
    }
}