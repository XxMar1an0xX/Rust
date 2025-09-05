use clearscreen;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, read};
use std::time::Duration;
const RESOLUTION: usize = 20;
const FONDO: char = '░';

enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
    Nulo,
}
enum Tipo {
    Estatico,
    Movimiento(),
}
struct Posicion {
    x: f64,
    y: f64,
}
impl Posicion {
    fn clone(&self) -> Self {
        Posicion {
            x: self.x,
            y: self.y,
        }
    }
}
struct Elemento {
    posicion: Posicion,
    direccion: Direccion,
    velocidad: f64,
    previa: Posicion,
}
impl Elemento {
    fn new() -> Self {
        Elemento {
            posicion: Posicion { x: 0.0, y: 0.0 },
            direccion: Direccion::Abajo,
            velocidad: 0.0,
            previa: Posicion { x: 0.0, y: 0.0 },
        }
    }
    fn actualizar(&mut self) {
        self.previa = self.posicion.clone();
        match self.direccion {
            Direccion::Arriba => {
                self.posicion.y -= self.velocidad;
                if self.posicion.y.round() <= -1.0 {
                    self.posicion.y = RESOLUTION as f64;
                }
            }
            Direccion::Abajo => {
                self.posicion.y += self.velocidad;
            }
            Direccion::Izquierda => {
                self.posicion.x -= self.velocidad;
                if self.posicion.x.round() <= -1.0 {
                    self.posicion.x = RESOLUTION as f64;
                }
            }
            Direccion::Derecha => {
                self.posicion.x += self.velocidad;
            }
            Direccion::Nulo => {
                panic!("Solo se podia salir asi...");
            }
        }
    }
    fn colisiona_con(&self, objeto: Elemento) -> bool {
        self.posicion.x == objeto.posicion.x && self.posicion.y == objeto.posicion.y
    }
}

struct Pantalla(Vec<Vec<char>>);
impl Pantalla {
    fn new() -> Self {
        let columnas = [FONDO; RESOLUTION].to_vec();
        let filas = vec![columnas; RESOLUTION];

        Pantalla(filas)
    }
    fn print(&self) {
        for columna in self.0.clone().into_iter() {
            for pixel in columna {
                print!("{}", pixel);
            }
            println!("\r");
        }
    }
    fn cambiar_pixel(&mut self, posicion: &Posicion, caracter: char) {
        let columna: &mut Vec<char> = &mut self.0[posicion.y.round() as usize % RESOLUTION];
        columna[posicion.x.round() as usize % RESOLUTION] = caracter;
    }
}

fn main() {
    let _ = crossterm::terminal::enable_raw_mode();
    let mut pantalla = Pantalla::new();
    let mut snake = vec![Elemento::new(), Elemento::new()];

    snake[0].velocidad = 1.0;
    snake[0].direccion = Direccion::Arriba;

    let mut buffer = snake[0].posicion.clone();
    loop {
        pantalla.print();
        if let Some(direccion) = entrada_controles() {
            snake[0].direccion = direccion;
        }
        // for element in &mut snake {
        //     element.actualizar();
        //     buffer = element.previa.clone();
        //     pantalla.cambiar_pixel(&element.posicion, 'o');
        // }
        for indice in (0..=snake.len() - 2) {
            snake[indice].actualizar();
            buffer = snake[indice].previa.clone();
            snake[indice + 1].posicion = buffer;
        }

        // pantalla.cambiar_pixel(&snake[0].posicion, 'o');
        pantalla.cambiar_pixel(&snake[snake.len() - 1].previa, FONDO);

        clearscreen::clear().expect("no funciono el borrado")
    }
}
fn entrada_controles() -> Option<Direccion> {
    if event::poll(Duration::from_millis(500)).expect("no") {
        return Some(match read().expect("error brutal") {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            }) => Direccion::Arriba,
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            }) => Direccion::Abajo,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            }) => Direccion::Izquierda,
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            }) => Direccion::Derecha,

            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            }) => Direccion::Nulo,
            _ => return None,
        });
    }
    None
}
