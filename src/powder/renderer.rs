use super::assets::Assets;
use super::state::atom::Element;
use super::state::Atoms;
use super::state::State;
use ggez::{graphics::*, Context, GameResult};
use strum::IntoEnumIterator;

type Point2 = glam::Vec2;

type Button = (Mesh, Text);
type Buttons = Vec<Button>;

#[derive(Debug)]
pub struct RendererError;

#[derive(Debug)]
pub struct Renderer {
    frame_sandbox: Rect,
    frame_fps: Rect,
    frame_element_selector: Rect,
    pub scaling_factor: i32,
    // TODO: use this to cache the sandbox mesh (and any other Drawables that don't need to be
    // regenerated every frame)
    // pub mesh_sandbox: Option<Mesh>,
}

impl Renderer {
    pub fn new(ctx: &Context, state: &State) -> Self {
        // figure that the sandbox should take 80% of the smaller screen dimension?
        // and assuming square for now
        let (win_w, win_h) = ctx.gfx.drawable_size();
        let sandbox_size_px;
        if win_w > win_h {
            sandbox_size_px = (win_h * 0.8) as i32;
        } else {
            sandbox_size_px = (win_w * 0.8) as i32;
        }
        let sandbox_size_px = sandbox_size_px - sandbox_size_px % state.parameters.sandbox_w;
        // calc scaling factor based on this -- do we still need it?
        let scaling_factor = sandbox_size_px / state.parameters.sandbox_w;

        // calc sandbox frame
        let frame_sandbox = Rect::new(
            (win_w - sandbox_size_px as f32) / 2.0,
            (win_h - sandbox_size_px as f32) / 2.0,
            sandbox_size_px as f32,
            sandbox_size_px as f32,
        );

        // calc fps frame
        let fps_w = 200f32;
        let fps_h = 20f32;
        let frame_fps = Rect::new(
            frame_sandbox.x + frame_sandbox.w,
            frame_sandbox.y - fps_h,
            fps_w,
            fps_h,
        );

        // calc element buttons
        let frame_element_selector = Rect::new(
            frame_sandbox.x + frame_sandbox.w + 10f32,
            frame_sandbox.y,
            100f32,
            frame_sandbox.h,
        );

        Renderer {
            frame_sandbox: frame_sandbox,
            frame_fps: frame_fps,
            scaling_factor: scaling_factor,
            frame_element_selector: frame_element_selector,
        }
    }

    pub fn get_scaling_factor(&self) -> i32 {
        self.scaling_factor
    }

    pub fn get_frame_sandbox(&self) -> Rect {
        self.frame_sandbox.clone()
    }

    fn draw_fps(&self, ctx: &mut Context, frame: Rect, font: &Option<String>) -> GameResult<Text> {
        let mut text = Text::new(TextFragment {
            text: format!("{:.2}", ctx.time.fps()),
            color: Some(Color::WHITE),
            font: font.clone(),
            scale: Some(PxScale::from(20.0)),
        });
        text.set_bounds(Point2::new(frame.w, 100.0));
        text.set_layout(TextLayout {
            h_align: TextAlign::End,
            v_align: TextAlign::Middle,
        });
        Ok(text)
    }

    fn draw_sandbox(&self, ctx: &mut Context, sandbox: Rect) -> GameResult<Mesh> {
        Ok(Mesh::from_data(
            ctx,
            MeshBuilder::new()
                .rectangle(
                    DrawMode::stroke(1f32),
                    Rect::new(0f32, 0f32, sandbox.w + 1.0, sandbox.h + 1.0),
                    Color::WHITE,
                )?
                .build(),
        ))
    }

    fn draw_button(
        &self,
        ctx: &mut Context,
        button: Rect,
        text_str: String,
        font: &Option<String>,
    ) -> Button {
        // button outline
        let outline = Mesh::from_data(
            ctx,
            MeshBuilder::new()
                .rectangle(
                    DrawMode::stroke(1f32),
                    Rect::new(0f32, 0f32, button.w + 1f32, button.h + 1f32),
                    Color::WHITE,
                )
                .expect("Couldn't draw button")
                .build(),
        );
        // button text
        let mut text = Text::new(TextFragment {
            text: text_str,
            color: Some(Color::WHITE),
            font: font.clone(),
            scale: Some(PxScale::from(10f32)),
        });
        text.set_bounds(Point2::new(button.x, button.y));
        text.set_layout(TextLayout {
            h_align: TextAlign::Begin,
            v_align: TextAlign::Middle,
        });

        return (outline, text);
    }

    fn draw_element_selector(&self, ctx: &mut Context, font: &Option<String>) -> Buttons {
        let mut element_selector: Vec<(Mesh, Text)> = vec![];
        // can't enumerate an enum so gotta keep an index separately
        let i = 0f32;
        for el in Element::iter() {
            let button_height = 20f32;
            let outline_rect = Rect {
                x: self.frame_element_selector.x,
                y: self.frame_element_selector.y + self.frame_element_selector.h
                    - i * (button_height + 10f32),
                w: self.frame_element_selector.w,
                h: button_height,
            };
            element_selector.push(self.draw_button(ctx, outline_rect, el.to_string(), font));
        }
        return element_selector;
    }

    fn draw_atoms(
        &self,
        ctx: &mut Context,
        atoms: &Atoms,
        scaling_factor: i32,
    ) -> GameResult<Mesh> {
        // TODO: proper co-ordinate conversion
        let mb = &mut MeshBuilder::new();
        for atom in atoms {
            let x = atom.coord.x * scaling_factor;
            let y = atom.coord.y * scaling_factor;
            mb.rectangle(
                DrawMode::fill(),
                Rect {
                    x: x as f32,
                    y: y as f32,
                    w: scaling_factor as f32,
                    h: scaling_factor as f32,
                },
                atom.color(),
            )
            .expect("Couldn't draw atom");
        }
        Ok(Mesh::from_data(ctx, mb.build()))
    }

    pub fn draw(&self, ctx: &mut Context, state: &State, assets: &Assets) -> GameResult {
        // refresh screen
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        // all drawing steps here
        let sandbox_m = self.draw_sandbox(ctx, self.frame_sandbox)?;
        let atoms_m = self.draw_atoms(ctx, state.get_atoms(), self.get_scaling_factor() as i32)?;
        let fps = self.draw_fps(ctx, self.frame_fps, &assets.font)?;
        // let button_water =
        canvas.draw(
            &sandbox_m,
            DrawParam::default().dest(Point2::new(self.frame_sandbox.x, self.frame_sandbox.y)),
        );
        canvas.draw(
            &atoms_m,
            DrawParam::default().dest(Point2::new(self.frame_sandbox.x, self.frame_sandbox.y)),
        );
        canvas.draw(
            &fps,
            DrawParam::default().dest(Point2::new(self.frame_fps.x, self.frame_fps.y)),
        );
        // output drawing
        canvas.finish(ctx)?;
        Ok(())
    }
}
