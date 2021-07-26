// This module should be purely functional - no internal state.
// Maybe this interface to main should be defined in a trait first?
// Regardless, event handlers get access to everything the relevant EventHandler method defines.

use super::state::*;
use ggez::*;

enum DrawError {
    OutOfSandboxError,
}

// helpers
fn convert_coord_to_sandbox_coord(
    settings: &settings::Settings,
    x: f32,
    y: f32,
) -> Result<SandboxCoordinate, DrawError> {
    if (x > settings.frame_sandbox.x)
        && (y > settings.frame_sandbox.y)
        && (x < (settings.frame_sandbox.x + settings.frame_sandbox.w))
        && (y < (settings.frame_sandbox.y + settings.frame_sandbox.h))
    {
        Ok(SandboxCoordinate::new(
            (x - settings.frame_sandbox.x) as u16,
            (y - settings.frame_sandbox.y) as u16,
        ))
    } else {
        Err(DrawError::OutOfSandboxError)
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
            println!("Handling LMB at ({},{})", x, y);
            match convert_coord_to_sandbox_coord(&state.settings, x, y) {
                Ok(coord) => {
                    println!("Making atom at ({}, {})", coord.x, coord.y);
                    // make atom
                    state.make_atom(coord, graphics::Color::WHITE)?;
                    Ok(())
                }
                Err(DrawError::OutOfSandboxError) => Ok(()),
            }
        }
        _ => Ok(()),
    }
}
