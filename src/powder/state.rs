#[derive(Copy, Clone)]
pub struct Atom {
    x: i32,
    y: i32,
}

impl Atom {
    pub fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub type Atoms = Vec<Atom>;

pub struct State {
    atoms: Atoms
}

impl State {
    pub fn new() -> Self {
        State {
            atoms: vec![]
        }
    }

    pub fn init(&mut self) {
        // stub init: make a single base atom
        self.atoms.push(
            Atom {
                x: 0,
                y: 0,
            }
        );
    }

    pub fn get_atoms(&self) -> &Atoms {
        &self.atoms
    }
}

