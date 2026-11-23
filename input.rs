
use crossterm::event::{
    self, Event, KeyCode, KeyEventKind,
};
use std::time::Duration;

#[derive(Default)]
pub struct Controles {
    pub izquierda: bool,
    pub derecha: bool,
    pub disparar: bool,
    pub salir: bool,
}

pub fn leer_controles(controles: &mut Controles) {
    while event::poll(Duration::from_millis(0)).unwrap_or(false) {
        if let Ok(Event::Key(tecla)) = event::read() {
            let presionada = match tecla.kind {
                KeyEventKind::Press | KeyEventKind::Repeat => true,
                KeyEventKind::Release => false,
            };

            match tecla.code {
                KeyCode::Left => {
                    controles.izquierda = presionada;
                }

                KeyCode::Right => {
                    controles.derecha = presionada;
                }

                KeyCode::Char(' ') => {
                    controles.disparar = presionada;
                }

                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    if presionada {
                        controles.salir = true;
                    }
                }

                _ => {}
            }
        }
    }
}
