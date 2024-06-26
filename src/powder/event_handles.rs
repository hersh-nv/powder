// This module should be purely functional - no internal state.
// Maybe this interface to main should be defined in a trait first?
// Regardless, event handlers get access to everything the relevant EventHandler method defines.

use anyhow::Error;

use super::state::*;
use super::Renderer;
use ggez::*;
use log::*;

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

fn convert_coord_to_sandbox_coord(renderer: &Renderer, x: f32, y: f32) -> SandboxCoordinate {
    // even though state.make_atom() checks invalid mutation, check here as well that we're not
    // getting underflows or anything silly
    SandboxCoordinate {
        x: (x - renderer.get_frame_sandbox().x) as i32 / renderer.get_scaling_factor(),
        y: (y - renderer.get_frame_sandbox().y) as i32 / renderer.get_scaling_factor(),
    }
}

// handlers
pub fn update(_ctx: &mut Context, _state: &mut State) -> Result<(), Error> {
    // debug!("Frame length: {}", timer::delta(ctx).as_millis());
    Ok(())
}

pub fn mouse_button_down_event(
    _ctx: &mut Context,
    state: &mut State,
    renderer: &Renderer,
    button: input::mouse::MouseButton,
    x: f32,
    y: f32,
) -> GameResult {
    match button {
        input::mouse::MouseButton::Left => {
            // handle LMB
            debug!("Handling LMB at ({}, {})", x, y);
            // TODO: probably need to move this to the renderer because it
            // shouldn't be the event handler's job to determine what ui has
            // been clicked
            if click_in_rect(x, y, renderer.get_frame_sandbox()) {
                // if clicked in sandbox
                let coord = convert_coord_to_sandbox_coord(&renderer, x, y);
                info!("Making atom at ({}, {})", coord.x, coord.y);
                state.make_atom(coord).map_err(|err| info!("{}", err)).ok();
                Ok(())
            } else if click_in_rect(x, y, renderer.get_frame_element_selector()) {
                // if clicked in element selector
                let coord = convert_coord_to_sandbox_coord(&renderer, x, y);
                info!("Clicked in element selector at ({}, {})", coord.x, coord.y);
                for button in renderer.get_buttons() {
                    if click_in_rect(x, y, button.rect) {
                        // if clicked on button
                        state.set_active_element(button.el);
                    }
                }
                Ok(())
            } else {
                // if clicked outside of sandbox
                debug!("EH: Atom out of bounds, not generating");
                return Ok(());
            }
        }
        _ => Ok(()),
    }
}
