use ggez::graphics::Color;

#[derive(Copy, Clone)]
pub struct Atom {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

pub type Atoms = Vec<Atom>;

pub struct State {
    atoms: Atoms,
}

impl State {
    pub fn new() -> Self {
        State { atoms: vec![] }
    }

    pub fn init(&mut self) {
        // stub init: make a single base atom
        self.atoms.push(Atom {
            x: 0,
            y: 0,
            color: Color::RED,
        });
        self.atoms.push(Atom {
            x: 10,
            y: 10,
            color: Color::GREEN,
        });
        self.atoms.push(Atom {
            x: 50,
            y: 50,
            color: Color::BLUE,
        });
    }

    pub fn get_atoms(&self) -> &Atoms {
        &self.atoms
    }
}
