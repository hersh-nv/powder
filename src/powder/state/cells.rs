use anyhow::Result;
use thiserror::Error;

use super::Atom;
use super::SandboxCoordinate;

#[derive(Error, Debug)]
pub enum CellsError {
    #[error("Atom could not be created at {x}, {y}")]
    CouldNotFillCell { x: i32, y: i32 },
}

#[derive(Clone)]
pub struct Cells {
    size: (i32, i32),
    array: Vec<Option<Atom>>,
}

// Cells keeps a vector of every cell in the drawable space, starting from
// (0,0)..(max_width,0) then continuing on to (0,1)..(max_width,1), all the way
// up to (max_width,max_height), so the overall array is max_width * max_height
// * cellsize in memory.
impl Cells {
    pub fn new(sandbox_size: i32) -> Self {
        Cells {
            size: (sandbox_size, sandbox_size),
            array: vec![None; (sandbox_size * sandbox_size) as usize],
        }
    }

    pub fn fill_cell(&mut self, atom: Atom) -> Result<()> {
        if let Some(atom_exists) = self.get_cell_contents(atom.coord) {
            Err(CellsError::CouldNotFillCell {
                x: atom_exists.coord.x,
                y: atom_exists.coord.y,
            }
            .into())
        } else {
            self.array[(atom.coord.y * self.size.0 + atom.coord.x) as usize] = Some(atom);
            Ok(())
        }
    }

    pub fn clear_cell(&mut self, coord: SandboxCoordinate) {
        self.array[(coord.y * self.size.0 + coord.x) as usize] = None;
    }
    pub fn get_cell_contents(&self, coord: SandboxCoordinate) -> Option<Atom> {
        self.array[(coord.y * self.size.0 + coord.x) as usize]
    }

    // fn get_atom_neighbourhood(&self, atom: Atom) -> Vec<bool> {
    //     let nh = vec![];
    //     for dy in -1..2 {
    //         for dx in -1..2 {
    //             if dx == 0 && dy == 0 {
    //                 continue;
    //             }
    //             let target = SandboxCoordinate {
    //                 x: atom.coord.x + dx,
    //                 y: atom.coord.y + dy,
    //             };
    //             if self.atom_out_of_bounds(target) || self.get(target) {
    //                 neighbourhood.push(true);
    //             } else {
    //                 neighbourhood.push(false);
    //             }
    //             self.get_cell_contents(coord)
    //         }
    //     }
    //     vec![false; 8]
    // }
}
