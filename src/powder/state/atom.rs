use log::debug;

use ggez::graphics::Color;

use super::SandboxCoordinate;

#[derive(Copy, Clone, Debug)]
pub enum Element {
    Sand,
    Water
}

// let Sand = Element { color: Color::WHITE }
impl Element {
    fn color(&self) -> Color {
        match self {
            Element::Sand => Color::WHITE,
            Element::Water => Color::BLUE,
        }
  }
  fn calculate_move(&self, neighbourhood: Vec<bool>) -> (i32, i32) {
    match neighbourhood[..] {
        [_, _, _, _, _, _, false, _] => (0, 1),
        [_, _, _, _, _, false, true, true] => (-1, 1),
        [_, _, _, _, _, true, true, false] => (1, 1),
        [_, _, _, _, _, false, true, false] => (rand::random::<bool>() as i32 * 2 - 1, -1),
        // yeah all the other coords are unused for now but could be useful later
        _ => (0, 0),
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Atom {
    element: Element,
    pub coord: SandboxCoordinate,
    pub next_coord: SandboxCoordinate,
}

impl Atom {
    pub fn new(coord: SandboxCoordinate, element: Element) -> Self {
        Atom { coord: coord, element: element, next_coord: coord}
    }

    pub fn color(&self) -> Color {
      self.element.color()
    }

    pub fn set_next(&mut self, neighbourhood: Vec<bool>) {
        // neighbourhood is a vec of surrounding coords in 1,2,3,4,6,7,8,9 order
        // as on a keypad (as if the updating atom is at pos 5), where the
        // element is true if that coord contains an atom and false if it
        // doesn't
        assert_eq!(neighbourhood.len(), 8);
        debug!("{self:?}, {neighbourhood:?}");
        let (dx, dy) = self.element.calculate_move(neighbourhood);
        self.next_coord.x = self.coord.x + dx;
        self.next_coord.y = self.coord.y + dy;
    }

    pub fn update(&mut self) {
        self.coord = self.next_coord;
    }
}
