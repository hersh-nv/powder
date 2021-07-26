use super::assets::Assets;
use super::state::Atoms;
use super::state::State;
use ggez::{graphics, graphics::*, timer, Context, GameResult};

type Point2 = glam::Vec2;

fn draw_fps(ctx: &mut Context, frame: Rect, font: Option<Font>) -> GameResult<Text> {
    let mut text = Text::new(TextFragment {
        text: format!("{:.2}", timer::fps(ctx)),
        color: Some(Color::WHITE),
        font: font,
        scale: Some(PxScale::from(20.0)),
    });
    text.set_bounds(Point2::new(frame.w, 100.0), Align::Right);
    Ok(text)
}

fn draw_atoms(ctx: &mut Context, sandbox: Rect, atoms: &Atoms) -> GameResult<Mesh> {
    // TODO: proper co-ordinate conversion
    let mb = &mut MeshBuilder::new();
    for atom in atoms {
        let x = atom.coord.x;
        let y = atom.coord.y;
        mb.rectangle(
            DrawMode::fill(),
            Rect {
                x: x as f32,
                y: y as f32,
                w: 1f32,
                h: 1f32,
            },
            atom.color,
        )
        .expect("Couldn't draw atom");
    }
    mb.build(ctx)
}

fn draw_sandbox(ctx: &mut Context, sandbox: Rect) -> GameResult<Mesh> {
    MeshBuilder::new()
        .rectangle(DrawMode::stroke(1f32), sandbox, Color::WHITE)?
        .build(ctx)
}

pub fn draw(ctx: &mut Context, state: &State, assets: &Assets) -> GameResult {
    // refresh screen
    graphics::clear(ctx, Color::BLACK);
    // all drawing steps here
    let sandbox_m = draw_sandbox(ctx, state.settings.frame_sandbox)?;
    let atoms_m = draw_atoms(ctx, state.settings.frame_sandbox, state.get_atoms())?;
    let text = draw_fps(ctx, state.settings.frame_fps, Some(assets.font))?;
    graphics::draw(ctx, &sandbox_m, DrawParam::default())?;
    graphics::draw(
        ctx,
        &atoms_m,
        DrawParam::default().dest(Point2::new(
            state.settings.frame_sandbox.x,
            state.settings.frame_sandbox.y,
        )),
    )?;
    graphics::draw(
        ctx,
        &text,
        DrawParam::default().dest(Point2::new(
            state.settings.frame_fps.x,
            state.settings.frame_fps.y,
        )),
    )?;
    // output drawing
    graphics::present(ctx)?;
    Ok(())
}
