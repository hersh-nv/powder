use ggez;
use ggez::{Context, GameResult, graphics, graphics::Rect};
use super::state;
use super::state::{Atom, Atoms};

fn draw_atoms(ctx: &mut Context, atoms: &Atoms) -> GameResult<graphics::Mesh> {
    println!("Drawing {} atoms", atoms.len());
    let mb = &mut graphics::MeshBuilder::new();
    for atom in atoms {
        let (x,y) = atom.get_pos();
        mb.rectangle(
            graphics::DrawMode::fill(),
            Rect {
                x: x as f32,
                y: y as f32,
                w: 5f32,
                h: 5f32,
            },
            graphics::Color::WHITE,
        ).expect("Couldn't draw atom");
    };
    mb.build(ctx)
}

pub fn draw(ctx: &mut Context, state: &state::State) -> GameResult {
    // refresh screen
    graphics::clear(ctx, graphics::Color::BLACK);
    // all drawing steps here
    let mesh = draw_atoms(ctx, state.get_atoms())?;
    // output drawing
    graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;
    graphics::present(ctx)?;
    Ok(())
}