use super::settings::*;
use super::state::State;
use super::assets::Assets;
use super::state::Atoms;
use ggez::{graphics, graphics::*, Context, GameResult, timer};
use glam;

type Point2 = glam::Vec2;

fn draw_fps(ctx: &mut Context, frame: Rect, font: Option<Font>) -> GameResult<Text> {
    let mut text = Text::new( TextFragment {
        text: format!("{:.2}", timer::fps(ctx)),
        color: Some(Color::WHITE),
        font: font,
        scale: Some(PxScale::from(20.0))
    });
    text.set_bounds(
        Point2::new(frame.w, 100.0),
        Align::Right,
    );
    Ok(text)
}

fn draw_atoms(
    ctx: &mut Context,
    sandbox: Rect,
    atoms: &Atoms,
) -> GameResult<Mesh> {
    // println!("Drawing {} atoms", atoms.len());
    let mb = &mut MeshBuilder::new();
    for atom in atoms {
        let x = atom.x + sandbox.x as i32;
        let y = atom.y + sandbox.y as i32;
        mb.rectangle(
            DrawMode::fill(),
            Rect {
                x: x as f32,
                y: y as f32,
                w: 5f32,
                h: 5f32,
            },
            atom.color,
        )
        .expect("Couldn't draw atom");
    }
    mb.build(ctx)
}

fn draw_sandbox(ctx: &mut Context, sandbox: Rect) -> GameResult<Mesh> {
    MeshBuilder::new()
        .rectangle(
            DrawMode::stroke(1f32),
            sandbox,
            Color::WHITE,
        )?
        .build(ctx)
}

pub fn draw(
    ctx: &mut Context, 
    settings: &Settings, 
    state: &State,
    assets: &Assets
) -> GameResult {
    // refresh screen
    graphics::clear(ctx, Color::BLACK);
    // graphics::set_screen_coordinates(ctx, graphics::Rect::new(0f32, 0f32, 512f32, 512f32))?;
    // all drawing steps here
    let sandbox_m = draw_sandbox(ctx, settings.sandbox)?;
    let atoms_m = draw_atoms(ctx, settings.sandbox, state.get_atoms())?;
    let text = draw_fps(ctx, settings.frame_fps, Some(assets.font))?;
    // output drawing
    graphics::draw(ctx, &sandbox_m, DrawParam::default())?;
    graphics::draw(ctx, &atoms_m, DrawParam::default())?;
    graphics::draw(ctx, &text, DrawParam::default()
        .dest(Point2::new(settings.frame_fps.x, settings.frame_fps.y)))?;
    graphics::present(ctx)?;
    Ok(())
}
