use log::debug;

use ggez::graphics::Color;

use super::SandboxCoordinate;

fn heads_or_tails() -> i32 {
    // Returns -1 or +1.
    if rand::random::<bool>() {
        1
    } else {
        -1
    }
}

fn heads_or_zip() -> i32 {
    // Returns -1 or +1.
    if rand::random::<bool>() {
        1
    } else {
        0
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Element {
    Sand,
    Water,
}

impl Element {
    fn color(&self) -> Color {
        match self {
            Element::Sand => Color::WHITE,
            Element::Water => Color::BLUE,
        }
    }

    fn calculate_move(&self, neighbourhood: Vec<bool>) -> (i32, i32) {
        let (dx, dy) = match self {
            Element::Sand => {
                match neighbourhood[..] {
                    [_, _, _, _, _, _, false, _] => (0, 1),
                    [_, _, _, _, _, false, true, true] => (-1, 1),
                    [_, _, _, _, _, true, true, false] => (1, 1),
                    [_, _, _, _, _, false, true, false] => (heads_or_tails(), 1),
                    // yeah all the other coords are unused for now but could be useful later
                    _ => (0, 0),
                }
            }
            Element::Water => {
                match neighbourhood[..] {
                    [_, _, _, _, _, _, false, _] => (0, 1),
                    [_, _, _, _, _, false, true, true] => (-1, 1),
                    [_, _, _, _, _, true, true, false] => (1, 1),
                    [_, _, _, _, _, false, true, false] => (heads_or_tails(), 1),
                    // for remaining cases we can assume coords below are full
                    [_, _, _, true, false, _, _, _] => (heads_or_zip(), 0),
                    [_, _, _, false, true, _, _, _] => (heads_or_zip() * -1, 0),
                    [_, _, _, false, false, _, _, _] => (heads_or_tails(), 0),
                    // yeah all the other coords are unused for now but could be useful later
                    _ => (0, 0),
                }
            }
        };
        assert!((-1..2).contains(&dx));
        assert!((-1..2).contains(&dy));
        (dx, dy)
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
        Atom {
            coord: coord,
            element: element,
            next_coord: coord,
        }
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
