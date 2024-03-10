use anyhow::Result;
use log::error;
use thiserror::Error;
use rand;

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
pub type SandboxCoordinate = Vector2<i32>;

#[derive(Copy, Clone)]
pub struct Atom {
    pub coord: SandboxCoordinate,
    pub color: Color,
}

impl Atom {
    pub fn update(&mut self, neighbourhood: Vec<bool>) -> Result<(), StateError> {
        // neighbourhood is a vec of surrounding coords in 1,2,3,4,6,7,8,9 order
        // as on a keypad (as if the updating atom is at pos 5), where the
        // element is true if that coord contains an atom and false if it
        // doesn't
        let (dx, dy) = match neighbourhood[..] {
            [_, _, _, _, _, _, false, _] => (0, -1),
            [_, _, _, _, _, false, true, true] => (-1, -1),
            [_, _, _, _, _, true, true, false] => (1, -1),
            [_, _, _, _, _, false, true, false] => (rand::random::<bool>() as i32 * 1, -1),
            // yeah all the other coords are unused for now but could be useful later
            _ => (0, 0),
        };
        Ok(())
    }
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
            });
            Ok(())
        }
    }

    pub fn get_atoms(&self) -> &Atoms {
        &self.atoms
    }

    fn get_atom_neighbourhood(&mut self, atom: Atom) -> Vec<bool> {
        let mut neighbourhood: Vec<bool> = vec![];
        for dx in -1..1 {
            for dy in -1..1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.atom_collision(SandboxCoordinate { x: atom.coord.x + dx, y: atom.coord.y + dy}) {
                    neighbourhood.push(true);
                } else {
                    neighbourhood.push(false);
                }
            }
        }
        assert!(neighbourhood.len() == 8);
        neighbourhood
    }
}
