use anyhow::Result;

use ggez::mint::Vector2;
use thiserror::Error;

pub mod parameters;
use parameters::*;
pub mod atom;
use atom::*;
pub mod cells;
use cells::*;

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
    pub parameters: Parameters,
    cells: Cells,
    atoms: Atoms,
    active_element: Element,
}

impl State {
    pub fn new(sandbox_size: i32) -> Self {
        State {
            parameters: Parameters::new(sandbox_size),
            atoms: vec![],
            cells: Cells::new(sandbox_size),
            active_element: Element::Sand,
        }
    }

    pub fn init(&mut self) {}

    pub fn set_active_element(&mut self, el: Element) {
        self.active_element = el;
    }

    fn atom_out_of_bounds(&self, coord: SandboxCoordinate) -> bool {
        coord.x < 0
            || coord.x >= self.parameters.sandbox_w
            || coord.y < 0
            || coord.y >= self.parameters.sandbox_h
    }

    fn atom_exists_here(&self, coord: SandboxCoordinate) -> bool {
        log::debug!("{coord:?}");
        if let Some(_) = self.cells.get_cell_contents(coord) {
            return true;
        }
        false
    }

    pub fn make_atom(&mut self, coord: SandboxCoordinate) -> Result<(), StateError> {
        if self.atom_out_of_bounds(coord) {
            Err(StateError::AtomError(String::from("Atom out of bounds")))
        } else if self.atom_exists_here(coord) {
            Err(StateError::AtomError(String::from(
                "Atom already exists here",
            )))
        } else {
            self.atoms.push(Atom::new(coord, self.active_element));
            self.cells
                .fill_cell(Atom::new(coord, self.active_element))
                .expect("Couldn't fill cell");
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
                let target = SandboxCoordinate {
                    x: atom.coord.x + dx,
                    y: atom.coord.y + dy,
                };
                if self.atom_out_of_bounds(target) || self.atom_exists_here(target) {
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
            // optim: if neighbourhood is obviously full dont bother doing anything
            if nh[3..7] != [true, true, true, true, true] {
                atom.set_next(nh);
            }
        }
        for atom in &mut self.atoms {
            // destination cell might have been filled by another atom
            // if so, don't move, and clear next coord
            if let Some(_) = self.cells.get_cell_contents(atom.next_coord) {
                atom.reset_next();
            } else {
                self.cells.clear_cell(atom.coord);
                atom.update();
                self.cells
                    .fill_cell(atom.clone())
                    .expect("Couldn't fill cell");
            }
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
        state.make_atom(SandboxCoordinate { x: 3, y: 3 }).ok();
        state.make_atom(SandboxCoordinate { x: 4, y: 4 }).ok();
        // these two should fall straight down
        state.update_atoms();
        if let Some(atom) = state.get_atoms().get(0) {
            assert_eq!(atom.coord, SandboxCoordinate { x: 3, y: 4 });
        }
        if let Some(atom) = state.get_atoms().get(1) {
            assert_eq!(atom.coord, SandboxCoordinate { x: 4, y: 5 });
        }
        // add a third atom under the top one
        state.make_atom(SandboxCoordinate { x: 3, y: 5 }).ok();
        // top one should now fall down to the left
        state.update_atoms();
        if let Some(atom) = state.get_atoms().get(0) {
            assert_eq!(atom.coord, SandboxCoordinate { x: 2, y: 5 });
        }
    }

    #[test]
    fn atom_collides_with_ground() {
        let mut state = State::new(5);
        state.make_atom(SandboxCoordinate { x: 2, y: 2 }).ok();
        state.update_atoms(); // should be at [2,3]
        state.update_atoms(); // should be at [2,4]
        state.update_atoms(); // should be at [2,4]
        if let Some(atom) = state.get_atoms().get(0) {
            assert_eq!(atom.coord, SandboxCoordinate { x: 2, y: 4 });
        }
    }

    #[test]
    fn cells_updates_in_correct_spot() {
        let mut state = State::new(5);
        state.make_atom(SandboxCoordinate { x: 2, y: 2 }).ok();
        assert!(state
            .cells
            .get_cell_contents(SandboxCoordinate { x: 2, y: 2 })
            .is_some());
        // after update, check that cells updates properly
        state.update_atoms();
        assert!(state
            .cells
            .get_cell_contents(SandboxCoordinate { x: 2, y: 2 })
            .is_none());
        assert!(state
            .cells
            .get_cell_contents(SandboxCoordinate { x: 2, y: 3 })
            .is_some());
    }
}
