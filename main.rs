
mod entidades;
mod input;

use crossterm::{
    cursor,
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use entidades::Juego;
use input::{leer_controles, Controles};
use std::io::stdout;
use std::time::Duration;

fn main() {
    let mut salida = stdout();

    if let Err(error) = terminal::enable_raw_mode() {
        eprintln!("No se pudo activar el teclado: {}", error);
        return;
    }

    if let Err(error) = execute!(
        salida,
        EnterAlternateScreen,
        cursor::Hide,
        Clear(ClearType::All),
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    ) {
        let _ = terminal::disable_raw_mode();
        eprintln!("No se pudo preparar la pantalla: {}", error);
        return;
    }

    let resultado = std::panic::catch_unwind(|| {
        let mut juego = Juego::new();
        let mut controles = Controles::default();

        loop {
            // Leer las teclas que están presionadas.
            leer_controles(&mut controles);

            if controles.salir {
                break;
            }

            // Actualizar movimiento y disparos.
            juego.actualizar_controles(&controles);

            juego.actualizar();
            juego.dibujar();

            std::thread::sleep(Duration::from_millis(50));
        }
    });

    let _ = execute!(
        salida,
        PopKeyboardEnhancementFlags,
        cursor::Show,
        LeaveAlternateScreen
    );

    let _ = terminal::disable_raw_mode();

    if resultado.is_err() {
        eprintln!("El juego finalizo inesperadamente.");
    }
}
