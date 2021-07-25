use ggez::graphics::Color;
use ggez::GameResult;

#[derive(Copy, Clone)]
pub struct SandboxCoordinate {
    pub x: u16,
    pub y: u16,
}

impl SandboxCoordinate {
    pub fn new(x: u16, y: u16) -> Self {
        SandboxCoordinate { x: x, y: y }
    }
}

#[derive(Copy, Clone)]
pub struct Atom {
    pub coord: SandboxCoordinate,
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
        self.make_atom(SandboxCoordinate::new(0, 0), Color::RED).ok();
        self.make_atom(SandboxCoordinate::new(10, 10), Color::BLUE).ok();
        self.make_atom(SandboxCoordinate::new(50, 50), Color::GREEN).ok();
    }

    pub fn make_atom(&mut self, coord: SandboxCoordinate, color: Color) -> GameResult {
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
