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
fn click_in_rect(x: f32, y: f32, rect: graphics::Rect) -> bool {
    x > rect.x && x < rect.x + rect.w && y > rect.y && y < rect.y + rect.h
}

fn convert_coord_to_sandbox_coord(
    settings: &settings::Settings,
    x: f32,
    y: f32,
) -> SandboxCoordinate {
    // even though state.make_atom() checks invalid mutation, check here as well that we're not
    // getting underflows or anything silly
    SandboxCoordinate {
        x: (x - settings.frame_sandbox.x) as u16 / settings.get_scaling_factor() as u16,
        y: (y - settings.frame_sandbox.y) as u16 / settings.get_scaling_factor() as u16,
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
            // handle LMB
            println!("Handling LMB at ({}, {})", x, y);
            // TODO: probably need to make this a match to determine which box got clicked
            if click_in_rect(x, y, state.settings.frame_sandbox) {
                // if clicked in sandbox
                let coord = convert_coord_to_sandbox_coord(&state.settings, x, y);
                println!("Making atom at ({}, {})", coord.x, coord.y);
                state
                    .make_atom(coord, graphics::Color::WHITE)
                    .unwrap_or_else(|err| {
                        println!("State error:\n  {}", err);
                    });
                Ok(())
            } else {
                // if clicked outside of sandbox
                println!("EH: Atom out of bounds, not generating");
                return Ok(());
            }
        }
        _ => Ok(()),
    }
}
