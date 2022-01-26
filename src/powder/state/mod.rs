use anyhow::Result;
use log::error;
use thiserror::Error;

use ggez::graphics::Color;
use ggez::mint::Vector2;
use ggez::Context;

pub mod settings;
use settings::*;

/* Module error */
#[derive(Error, Debug)]
pub enum StateError {
    #[error("Couldn't change atom state: {0}")]
    AtomError(String),
}

/* Subtypes and structs */
pub type SandboxCoordinate = Vector2<u16>;

#[derive(Copy, Clone)]
pub struct Atom {
    pub coord: SandboxCoordinate,
    pub color: Color,
    pub vel: Vector2<u16>,
}

impl Atom {
    pub fn change_pos(&mut self, d: Vector2<u16>) -> Result<(), StateError> {
        self.coord.x = self.coord.x + d.x;
        self.coord.y = self.coord.y + d.y;
        Ok(())
    }
}

pub type Atoms = Vec<Atom>;

// pub type Field = [Vector2; ]

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
        self.make_atom(SandboxCoordinate { x: 0, y: 0 }, Color::RED)
            .ok();
        self.make_atom(SandboxCoordinate { x: 10, y: 10 }, Color::BLUE)
            .ok();
        self.make_atom(SandboxCoordinate { x: 20, y: 20 }, Color::GREEN)
            .ok();
    }

    fn atom_in_bounds(&self, coord: SandboxCoordinate) -> bool {
        !(coord.x > self.settings.sandbox_w || coord.y > self.settings.sandbox_h)
    }

    fn atom_collision(&self, coord: SandboxCoordinate) -> bool {
        // TODO: optimise
        for atom in &self.atoms {
            if coord.x != atom.coord.x {
                continue;
            } else {
                if coord.y != atom.coord.y {
                    continue;
                } else {
                    return true;
                }
            }
        }
        false
    }

    pub fn make_atom(&mut self, coord: SandboxCoordinate, color: Color) -> Result<(), StateError> {
        if !self.atom_in_bounds(coord) {
            Err(StateError::AtomError(String::from("Atom out of bounds")))
        } else if self.atom_collision(coord) {
            Err(StateError::AtomError(String::from(
                "Atom already exists here",
            )))
        } else {
            self.atoms.push(Atom {
                coord: coord,
                color: color,
                vel: Vector2 { x: 0, y: 0 },
            });
            Ok(())
        }
    }

    pub fn get_atoms(&self) -> &Atoms {
        &self.atoms
    }

    pub fn get_atoms_mut(&mut self) -> &mut Atoms {
        &mut self.atoms
    }

    pub fn apply_field(&mut self) {
        // TODO
    }
}
