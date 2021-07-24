use super::state;
use super::state::Atoms;
use super::settings::*;
use ggez;
use ggez::{graphics, graphics::Rect, Context, GameResult};

fn draw_atoms(ctx: &mut Context, atoms: &Atoms, sandbox: graphics::Rect) -> GameResult<graphics::Mesh> {
    println!("Drawing {} atoms", atoms.len());
    let mb = &mut graphics::MeshBuilder::new();
    for atom in atoms {
        let (x, y) = atom.get_pos();
        let x = x + sandbox.x as i32;
        let y = y + sandbox.y as i32;
        mb.rectangle(
            graphics::DrawMode::fill(),
            Rect {
                x: x as f32,
                y: y as f32,
                w: 5f32,
                h: 5f32,
            },
            graphics::Color::WHITE,
        )
        .expect("Couldn't draw atom");
    }
    mb.build(ctx)
}

fn draw_sandbox(ctx: &mut Context, sandbox: graphics::Rect) -> GameResult<graphics::Mesh> {
    println!("Drawing sandbox");
    graphics::MeshBuilder::new()
        .rectangle(
            graphics::DrawMode::stroke(1f32),
            sandbox, 
            graphics::Color::WHITE
        )?
        .build(ctx)
}

pub fn draw(ctx: &mut Context, settings: &Settings, state: &state::State) -> GameResult {
    // refresh screen
    graphics::clear(ctx, graphics::Color::BLACK);
    // graphics::set_screen_coordinates(ctx, graphics::Rect::new(0f32, 0f32, 512f32, 512f32))?;
    // all drawing steps here
    let sandbox_m = draw_sandbox(ctx, settings.sandbox)?;
    let atoms_m = draw_atoms(ctx, state.get_atoms(), settings.sandbox)?;
    // output drawing
    graphics::draw(ctx, &sandbox_m, graphics::DrawParam::new())?;
    graphics::draw(ctx, &atoms_m, graphics::DrawParam::new())?;
    graphics::present(ctx)?;
    Ok(())
}
