use clearscreen;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, read};
use rand::{self, Rng, rng};
use std::time::Duration;
const RESOLUTION: usize = 20;
const FONDO: char = '░';

#[derive(PartialEq)]
enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
    Nulo,
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
    fn colisiona_con(&self, objeto: &Posicion) -> bool {
        self.posicion.x == objeto.x && self.posicion.y == objeto.y
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
    let mut snake = vec![Elemento::new()];
    let mut listadeposiciones: Vec<Posicion> = Vec::new();
    for _ in 0..rng().random_range(1..=12) {
        let manzana = Posicion {
            x: rng().random_range(0..=20) as f64,
            y: rng().random_range(0..=20) as f64,
        };
        pantalla.cambiar_pixel(&manzana, 'M');
        listadeposiciones.push(manzana);
    }

    snake[0].velocidad = 1.0;
    snake[0].direccion = Direccion::Arriba;
    for _ in 1..rng().random_range(2..=6) {
        snake.push(Elemento::new());
    }

    loop {
        pantalla.print();
        if let Some(direccion) = entrada_controles() {
            if !girode180(&snake[0].direccion, &direccion) {
                snake[0].direccion = direccion;
            }
        }
        if listadeposiciones
            .iter()
            .any(|pos| snake[0].colisiona_con(pos))
        {
            snake.push(Elemento::new());
        }
        snake[0].actualizar();
        pantalla.cambiar_pixel(&snake[0].posicion, 's');

        let mut buffer = snake[0].previa.clone();
        for indice in 1..snake.len() {
            snake[indice].actualizar();
            snake[indice].posicion = buffer.clone();
            buffer = snake[indice].previa.clone();
            pantalla.cambiar_pixel(&snake[indice].posicion, 'o');
        }
        // pantalla.cambiar_pixel(&snake[0].posicion, 'o');
        pantalla.cambiar_pixel(&snake[snake.len() - 1].previa, FONDO);

        clearscreen::clear().expect("no funciono el borrado")
    }
}
fn entrada_controles() -> Option<Direccion> {
    if event::poll(Duration::from_millis(400)).expect("no") {
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
fn girode180(base: &Direccion, asignado: &Direccion) -> bool {
    match (base, asignado) {
        (Direccion::Arriba, Direccion::Abajo) => true,
        (Direccion::Abajo, Direccion::Arriba) => true,
        (Direccion::Derecha, Direccion::Izquierda) => true,
        (Direccion::Izquierda, Direccion::Derecha) => true,
        _ => false,
    }
}
