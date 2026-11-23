
use crate::input::Controles;

const ANCHO: i32 = 40;
const ALTO: i32 = 15;

pub struct Jugador {
    pub x: i32,
    pub vidas: i32,
}

impl Jugador {
    pub fn new() -> Jugador {
        Jugador {
            x: ANCHO / 2,
            vidas: 3,
        }
    }

    pub fn mover_izquierda(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn mover_derecha(&mut self) {
        if self.x < ANCHO - 1 {
            self.x += 1;
        }
    }
}

pub struct Enemigo {
    pub x: i32,
    pub y: i32,
}

impl Enemigo {
    pub fn new(x: i32, y: i32) -> Enemigo {
        Enemigo { x, y }
    }
}

pub struct Disparo {
    pub x: i32,
    pub y: i32,
}

impl Disparo {
    pub fn new(x: i32, y: i32) -> Disparo {
        Disparo { x, y }
    }
}

pub struct Juego {
    pub jugador: Jugador,
    pub enemigos: Vec<Enemigo>,
    pub disparos: Vec<Disparo>,
    pub disparos_enemigos: Vec<Disparo>,
    pub puntuacion: u32,
    pub direccion: i32,
    pub terminado: bool,
    pub victoria: bool,

    contador_enemigos: u32,
    contador_disparos: u32,
    espera_disparo: u32,
    espera_disparo_enemigo: u32,
    semilla: u32,
}

impl Juego {
    pub fn new() -> Juego {
        Juego {
            jugador: Jugador::new(),
            enemigos: Self::crear_enemigos(),
            disparos: Vec::new(),
            disparos_enemigos: Vec::new(),
            puntuacion: 0,
            direccion: 1,
            terminado: false,
            victoria: false,

            contador_enemigos: 0,
            contador_disparos: 0,
            espera_disparo: 0,
            espera_disparo_enemigo: 20,
            semilla: 123456789,
        }
    }

    fn crear_enemigos() -> Vec<Enemigo> {
        let mut enemigos = Vec::new();

        for fila in 0..2 {
            for columna in 0..8 {
                enemigos.push(Enemigo::new(
                    3 + columna * 4,
                    1 + fila * 2,
                ));
            }
        }

        enemigos
    }

    // Generador pseudoaleatorio.
    fn numero_azar(&mut self, maximo: u32) -> u32 {
        self.semilla = self.semilla
            .wrapping_mul(1664525)
            .wrapping_add(1013904223);

        self.semilla % maximo
    }

    // Disparo del jugador.
    pub fn disparar(&mut self) {
        if !self.terminado
            && self.disparos.len() < 3
            && self.espera_disparo == 0
        {
            self.disparos.push(Disparo::new(
                self.jugador.x,
                ALTO - 2,
            ));

            self.espera_disparo = 5;
        }
    }

    // Procesar los controles mantenidos.
    pub fn actualizar_controles(&mut self, controles: &Controles) {
        if self.terminado {
            return;
        }

        if controles.izquierda {
            self.jugador.mover_izquierda();
        }

        if controles.derecha {
            self.jugador.mover_derecha();
        }

        // Solo dispara cuando se presiona espacio.
        if controles.disparar {
            self.disparar();
        }
    }

    pub fn actualizar(&mut self) {
        if self.terminado {
            return;
        }

        // Tiempo entre disparos del jugador.
        if self.espera_disparo > 0 {
            self.espera_disparo -= 1;
        }

        // Tiempo entre disparos enemigos.
        if self.espera_disparo_enemigo > 0 {
            self.espera_disparo_enemigo -= 1;
        }

        // Elegir un enemigo al azar.
        if self.espera_disparo_enemigo == 0
            && !self.enemigos.is_empty()
        {
            let indice = self.numero_azar(
                self.enemigos.len() as u32
            ) as usize;

            let x = self.enemigos[indice].x;
            let y = self.enemigos[indice].y;

            self.disparos_enemigos.push(
                Disparo::new(x, y + 1)
            );

            self.espera_disparo_enemigo =
                15 + self.numero_azar(26);
        }

        // Mover disparos cada 100 milisegundos.
        self.contador_disparos += 1;

        if self.contador_disparos >= 2 {
            self.contador_disparos = 0;

            // Disparos del jugador hacia arriba.
            for disparo in &mut self.disparos {
                disparo.y -= 1;
            }

            // Comprobar impactos del jugador.
            let mut i = self.disparos.len();

            while i > 0 {
                i -= 1;

                let x = self.disparos[i].x;
                let y = self.disparos[i].y;

                let impacto = self.enemigos
                    .iter()
                    .position(|e| e.x == x && e.y == y);

                if let Some(indice) = impacto {
                    self.enemigos.remove(indice);
                    self.disparos.remove(i);
                    self.puntuacion += 10;
                } else if y < 0 {
                    self.disparos.remove(i);
                }
            }

            // Disparos enemigos hacia abajo.
            for disparo in &mut self.disparos_enemigos {
                disparo.y += 1;
            }

            // Comprobar impactos contra el jugador.
            let mut i = self.disparos_enemigos.len();

            while i > 0 {
                i -= 1;

                let x = self.disparos_enemigos[i].x;
                let y = self.disparos_enemigos[i].y;

                if x == self.jugador.x && y == ALTO - 1 {
                    self.disparos_enemigos.remove(i);
                    self.jugador.vidas -= 1;

                    if self.jugador.vidas <= 0 {
                        self.terminado = true;
                    }

                    break;
                } else if y >= ALTO {
                    self.disparos_enemigos.remove(i);
                }
            }
        }

        if self.terminado {
            return;
        }

        // Comprobar victoria.
        if self.enemigos.is_empty() {
            self.terminado = true;
            self.victoria = true;
            return;
        }

        // Movimiento de enemigos.
        self.contador_enemigos += 1;

        if self.contador_enemigos >= 7 {
            self.contador_enemigos = 0;

            let cambiar_direccion = self.enemigos.iter().any(|e| {
                (self.direccion == 1 && e.x >= ANCHO - 1)
                    || (self.direccion == -1 && e.x <= 0)
            });

            if cambiar_direccion {
                self.direccion *= -1;

                for enemigo in &mut self.enemigos {
                    enemigo.y += 1;
                }
            } else {
                for enemigo in &mut self.enemigos {
                    enemigo.x += self.direccion;
                }
            }

            // Si los enemigos llegan abajo.
            if self.enemigos.iter().any(|e| e.y >= ALTO - 2) {
                self.jugador.vidas -= 1;

                if self.jugador.vidas <= 0 {
                    self.terminado = true;
                } else {
                    self.enemigos = Self::crear_enemigos();
                    self.direccion = 1;
                }
            }
        }
    }

    pub fn dibujar(&self) {
        use crossterm::{
            cursor::MoveTo,
            queue,
            style::{Color, Print, ResetColor, SetForegroundColor},
            terminal::{Clear, ClearType},
        };

        use std::io::{stdout, Write};

        let mut salida = stdout();

        let (columnas, filas) =
            crossterm::terminal::size().unwrap_or((80, 24));

        let ancho_tablero = (ANCHO + 2) as u16;
        let alto_tablero = (ALTO + 7) as u16;

        let inicio_x = columnas.saturating_sub(ancho_tablero) / 2;
        let inicio_y = filas.saturating_sub(alto_tablero) / 2;

        let _ = queue!(salida, Clear(ClearType::All));

        let _ = queue!(
            salida,
            MoveTo(inicio_x, inicio_y),
            SetForegroundColor(Color::Cyan),
            Print("          SPACE INVADERS"),
            ResetColor
        );

        for y in 0..ALTO {
            let mut linea = String::new();

            for x in 0..ANCHO {
                let simbolo =
                    if self.enemigos.iter().any(|e| e.x == x && e.y == y) {
                        'W'
                    } else if self.disparos_enemigos.iter().any(
                        |d| d.x == x && d.y == y
                    ) {
                        'v'
                    } else if self.disparos.iter().any(
                        |d| d.x == x && d.y == y
                    ) {
                        '|'
                    } else if self.jugador.x == x && y == ALTO - 1 {
                        'A'
                    } else {
                        ' '
                    };

                linea.push(simbolo);
            }

            let _ = queue!(
                salida,
                MoveTo(inicio_x, inicio_y + 2 + y as u16),
                SetForegroundColor(Color::Green),
                Print(format!("|{}|", linea)),
                ResetColor
            );
        }

        let info_y = inicio_y + ALTO as u16 + 3;

        let _ = queue!(
            salida,
            MoveTo(inicio_x, info_y),
            SetForegroundColor(Color::Yellow),
            Print(format!(
                "PUNTOS: {}   VIDAS: {}   ENEMIGOS: {}",
                self.puntuacion,
                self.jugador.vidas,
                self.enemigos.len()
            )),
            MoveTo(inicio_x, info_y + 1),
            Print("IZQUIERDA / DERECHA: MOVER"),
            MoveTo(inicio_x, info_y + 2),
            Print("ESPACIO: DISPARAR    Q: SALIR"),
            ResetColor
        );

        if self.terminado {
            let mensaje = if self.victoria {
                "¡GANASTE!"
            } else {
                "GAME OVER"
            };

            let _ = queue!(
                salida,
                MoveTo(inicio_x, info_y + 4),
                SetForegroundColor(Color::Red),
                Print(mensaje),
                MoveTo(inicio_x, info_y + 5),
                Print(format!(
                    "Puntuacion final: {}",
                    self.puntuacion
                )),
                ResetColor
            );
        }

        let _ = salida.flush();
    }
}
