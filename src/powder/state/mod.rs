use ggez::graphics::Color;
use ggez::{Context, GameResult};
use ggez::mint::Vector2;

pub mod settings;
use settings::*;

/* Module error */
#[derive(Debug)]
pub struct StateError;

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Couldn't make this state change")
    }
}

/* Subtypes and structs */
pub type SandboxCoordinate = Vector2<u16>;

#[derive(Copy, Clone)]
pub struct Atom {
    pub coord: SandboxCoordinate,
    pub color: Color,
}

pub type Atoms = Vec<Atom>;

/* State */
pub struct State {
    pub settings: Settings,
    atoms: Atoms,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        State {
            settings: Settings::new(ctx),
            atoms: vec![],
        }
    }

    pub fn init(&mut self) {
        // stub init: make some test atoms
        self.make_atom(SandboxCoordinate {x: 0, y: 0}, Color::RED)
            .ok();
        self.make_atom(SandboxCoordinate {x: 0, y: 0}, Color::BLUE)
            .ok();
        self.make_atom(SandboxCoordinate {x: 0, y: 0}, Color::GREEN)
            .ok();
    }

    pub fn make_atom(&mut self, coord: SandboxCoordinate, color: Color) -> Result<(), StateError> {
        self.atoms.push(Atom {
            coord: coord,
            color: color,
        });
        Ok(())
    }

    pub fn get_atoms(&self) -> &Atoms {
        &self.atoms
    }
}
