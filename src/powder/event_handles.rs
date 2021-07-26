// This module should be purely functional - no internal state.
// Maybe this interface to main should be defined in a trait first?
// Regardless, event handlers get access to everything the relevant EventHandler method defines.

use super::state::*;
use ggez::*;

#[derive(Debug)]
pub struct EventHandlerError;

impl std::fmt::Display for EventHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Couldn't handle event")
    }
}

// helpers
fn convert_coord_to_sandbox_coord(
    settings: &settings::Settings,
    x: f32,
    y: f32,
) -> SandboxCoordinate {
    SandboxCoordinate {
        x: (x - settings.frame_sandbox.x) as u16,
        y: (y - settings.frame_sandbox.y) as u16,
    }
}

// handlers
pub fn mouse_button_down_event(
    ctx: &mut Context,
    state: &mut State,
    button: input::mouse::MouseButton,
    x: f32,
    y: f32,
) -> GameResult {
    match button {
        input::mouse::MouseButton::Left => {
            println!("Handling LMB at ({}, {})", x, y);
            let coord = convert_coord_to_sandbox_coord(&state.settings, x, y);
            println!("Making atom at ({}, {})", coord.x, coord.y);
            // make atom
            state
                .make_atom(coord, graphics::Color::WHITE)
                .unwrap_or_else(|_| {
                    println!("Atom out of bounds, not generating");
                });
            Ok(())
        }
        _ => Ok(()),
    }
}
