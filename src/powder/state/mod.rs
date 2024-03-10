use anyhow::Result;
#[cfg(not(test))]
use log::{debug, info};

#[cfg(test)]
use std::{println as debug, println as info};

use thiserror::Error;
use rand;

use ggez::graphics::Color;
use ggez::mint::Vector2;

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

#[derive(Copy, Clone, Debug)]
pub struct Atom {
    pub coord: SandboxCoordinate,
    pub color: Color,
    pub next_coord: SandboxCoordinate,
}

impl Atom {
    pub fn new(coord: SandboxCoordinate, color: Color) -> Self {
        Atom { coord: coord, color: color, next_coord: coord}
    }
    
    pub fn set_next(&mut self, neighbourhood: Vec<bool>) {
        // neighbourhood is a vec of surrounding coords in 1,2,3,4,6,7,8,9 order
        // as on a keypad (as if the updating atom is at pos 5), where the
        // element is true if that coord contains an atom and false if it
        // doesn't
        let (dx, dy) = match neighbourhood[..] {
            [_, _, _, _, _, _, false, _] => (0, 1),
            [_, _, _, _, _, false, true, true] => (-1, 1),
            [_, _, _, _, _, true, true, false] => (1, 1),
            [_, _, _, _, _, false, true, false] => (rand::random::<bool>() as i32 * 2 - 1, -1),
            // yeah all the other coords are unused for now but could be useful later
            _ => (0, 0),
        };
        self.next_coord.x = self.coord.x + dx;
        self.next_coord.y = self.coord.y + dy;
        debug!("{self:?}, {neighbourhood:?}");
    }

    pub fn update(&mut self) {
        self.coord = self.next_coord;
    }
}

pub type Atoms = Vec<Atom>;

/* State */
#[derive(Clone)]
pub struct State {
    pub settings: Settings,
    atoms: Atoms,
}

impl State {
    pub fn new(sandbox_size: i32) -> Self {
        State {
            settings: Settings::new(sandbox_size),
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

    fn atom_out_of_bounds(&self, coord: SandboxCoordinate) -> bool {
        coord.x > self.settings.sandbox_w || coord.y > self.settings.sandbox_h
    }

    fn atom_collision(&self, coord: SandboxCoordinate) -> bool {
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
        if self.atom_out_of_bounds(coord) {
            Err(StateError::AtomError(String::from("Atom out of bounds")))
        } else if self.atom_collision(coord) {
            Err(StateError::AtomError(String::from(
                "Atom already exists here",
            )))
        } else {
            self.atoms.push(Atom::new(coord, color));
            Ok(())
        }
    }

    pub fn get_atoms(&self) -> &Atoms {
        &self.atoms
    }

    fn get_atom_neighbourhood(&self, atom: &Atom) -> Vec<bool> {
        let mut neighbourhood: Vec<bool> = vec![];
        for dy in -1..2 {
            for dx in -1..2 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let target = SandboxCoordinate { x: atom.coord.x + dx, y: atom.coord.y + dy};
                if self.atom_collision(target) || self.atom_out_of_bounds(target) {
                    neighbourhood.push(true);
                } else {
                    neighbourhood.push(false);
                }
            }
        }
        assert_eq!(neighbourhood.len(), 8);
        neighbourhood
    }

    pub fn update_atoms(&mut self) {
        // let atoms_copy = self.atoms.clone();
        let self_copy = self.clone();
        for atom in &mut self.atoms {
            let nh = self_copy.get_atom_neighbourhood(atom);
            atom.set_next(nh);
        }
        for atom in &mut self.atoms {
            atom.update();
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_and_update_three_atoms() {
        // init with two atoms
        let mut state = State::new(10);
        state.make_atom(SandboxCoordinate {x: 3, y: 3}, Color::RED).ok();
        state.make_atom(SandboxCoordinate {x: 4, y: 4}, Color::BLUE).ok();
        // these two should fall straight down
        state.update_atoms();
        if let Some(atom) = state.get_atoms().get(0) {
            assert_eq!(atom.coord, SandboxCoordinate{x: 3, y: 4});
        }
        if let Some(atom) = state.get_atoms().get(1) {
            assert_eq!(atom.coord, SandboxCoordinate{x: 4, y: 5});                
        }
        // add a third atom under the top one
        state.make_atom(SandboxCoordinate {x: 3, y: 5}, Color::GREEN).ok();
        // top one should now fall down to the left
        state.update_atoms();
        if let Some(atom) = state.get_atoms().get(0) {
            assert_eq!(atom.coord, SandboxCoordinate{x: 2, y: 5});
        }
    }

    #[test]
    fn atom_collides_with_ground() {
        let mut state = State::new(5);
        state.make_atom(SandboxCoordinate{ x: 3, y: 3}, Color::RED).ok();
        state.update_atoms(); // should be at [3,4]
        state.update_atoms(); // should be at [3,5]
        state.update_atoms(); // should be at [3,5]
        if let Some(atom) = state.get_atoms().get(0) {
            assert_eq!(atom.coord, SandboxCoordinate{x: 3, y: 5});
        }
    }
}
