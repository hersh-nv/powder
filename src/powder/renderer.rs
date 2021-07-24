use ggez::{Context, GameResult, graphics};
use super::state;

fn draw_atoms(ctx: &mut Context, obj_state: &state::ObjState, ) {

}

pub fn draw(ctx: &mut Context, state: &state::ObjState) -> GameResult {
    // refresh screen
    graphics::clear(ctx, graphics::Color::BLACK);
    // all drawing steps here
    draw_atoms(ctx, state);
    // output drawing
    graphics::present(ctx);
    Ok(())
}