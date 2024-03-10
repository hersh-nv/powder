use anyhow::Result;
use log::{debug, info};

use thiserror::Error;
use ggez::mint::Vector2;

pub mod settings;
use settings::*;
pub mod atom;
use atom::*;

pub type SandboxCoordinate = Vector2<i32>;

/* Module error */
#[derive(Error, Debug)]
pub enum StateError {
    #[error("Couldn't change atom state: {0}")]
    AtomError(String),
}

/* Subtypes and structs */
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
        self.make_atom(SandboxCoordinate { x: 0, y: 0 }, Element::Sand)
            .ok();
        self.make_atom(SandboxCoordinate { x: 10, y: 10 }, Element::Sand)
            .ok();
        self.make_atom(SandboxCoordinate { x: 20, y: 20 }, Element::Sand)
            .ok();
    }

    fn atom_out_of_bounds(&self, coord: SandboxCoordinate) -> bool {
        coord.x >= self.settings.sandbox_w || coord.y >= self.settings.sandbox_h
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

    pub fn make_atom(&mut self, coord: SandboxCoordinate, element: Element) -> Result<(), StateError> {
        if self.atom_out_of_bounds(coord) {
            Err(StateError::AtomError(String::from("Atom out of bounds")))
        } else if self.atom_collision(coord) {
            Err(StateError::AtomError(String::from(
                "Atom already exists here",
            )))
        } else {
            self.atoms.push(Atom::new(coord, element));
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
        state.make_atom(SandboxCoordinate {x: 3, y: 3}, Element::Sand).ok();
        state.make_atom(SandboxCoordinate {x: 4, y: 4}, Element::Sand).ok();
        // these two should fall straight down
        state.update_atoms();
        if let Some(atom) = state.get_atoms().get(0) {
            assert_eq!(atom.coord, SandboxCoordinate{x: 3, y: 4});
        }
        if let Some(atom) = state.get_atoms().get(1) {
            assert_eq!(atom.coord, SandboxCoordinate{x: 4, y: 5});                
        }
        // add a third atom under the top one
        state.make_atom(SandboxCoordinate {x: 3, y: 5}, Element::Sand).ok();
        // top one should now fall down to the left
        state.update_atoms();
        if let Some(atom) = state.get_atoms().get(0) {
            assert_eq!(atom.coord, SandboxCoordinate{x: 2, y: 5});
        }
    }

    #[test]
    fn atom_collides_with_ground() {
        let mut state = State::new(5);
        state.make_atom(SandboxCoordinate{ x: 2, y: 2}, Element::Sand).ok();
        state.update_atoms(); // should be at [2,3]
        state.update_atoms(); // should be at [2,4]
        state.update_atoms(); // should be at [2,4]
        if let Some(atom) = state.get_atoms().get(0) {
            assert_eq!(atom.coord, SandboxCoordinate{x: 2, y: 4});
        }
    }
}
