pub struct Atom {
    x: i32,
    y: i32,
}

pub struct ObjState {
    atoms: Vec<Atom>
}

impl ObjState {
    pub fn new() -> Self {
        ObjState {
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
}

