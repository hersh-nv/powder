use super::assets::Assets;
use super::state::atom::Element;
use super::state::Atoms;
use super::state::State;
use ggez::{graphics::*, Context, GameResult};
use strum::IntoEnumIterator;

type Point2 = glam::Vec2;

#[derive(Debug, Clone)]
pub struct Button {
    frame: Mesh,
    text: Text,
    pub rect: Rect,
    pub el: Element,
}
type Buttons = Vec<Button>;

#[derive(Debug)]
pub struct RendererError;

#[derive(Debug)]
pub struct Renderer {
    frame_sandbox: Rect,
    frame_fps: Rect,
    frame_element_selector: Rect,
    font: Option<String>,
    pub scaling_factor: i32,
    // TODO: use this to cache the sandbox mesh (and any other Drawables that don't need to be
    // regenerated every frame)
    mesh_sandbox: Option<Mesh>,
    buttons: Option<Buttons>,
}

impl Renderer {
    pub fn new(ctx: &Context, state: &State, font: Option<String>) -> Self {
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
            (win_w - sandbox_size_px as f32) / 2f32,
            (win_h - sandbox_size_px as f32) / 2f32,
            sandbox_size_px as f32 + 1f32,
            sandbox_size_px as f32 + 1f32,
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
            80f32,
            frame_sandbox.h,
        );

        Renderer {
            frame_sandbox: frame_sandbox,
            frame_fps: frame_fps,
            frame_element_selector: frame_element_selector,
            font: font,
            scaling_factor: scaling_factor,
            mesh_sandbox: None,
            buttons: None,
        }
    }

    pub fn init(&mut self, ctx: &mut Context) {
        // some drawables don't have to be redrawn every time, so are saved in the renderer at init
        self.mesh_sandbox = Some(self.draw_sandbox(ctx, self.frame_sandbox));
        self.buttons = Some(self.draw_element_selector(ctx));
    }

    pub fn get_scaling_factor(&self) -> i32 {
        self.scaling_factor
    }

    pub fn get_frame_sandbox(&self) -> Rect {
        self.frame_sandbox.clone()
    }

    pub fn get_frame_element_selector(&self) -> Rect {
        self.frame_element_selector.clone()
    }

    pub fn get_buttons(&self) -> Buttons {
        if let Some(buttons) = self.buttons.clone() {
            return buttons;
        } else {
            return vec![];
        }
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

    fn draw_sandbox(&self, ctx: &mut Context, sandbox: Rect) -> Mesh {
        Mesh::from_data(
            ctx,
            MeshBuilder::new()
                .rectangle(
                    DrawMode::stroke(1f32),
                    Rect::new(0f32, 0f32, sandbox.w, sandbox.h),
                    Color::WHITE,
                )
                .expect("Couldn't draw sandbox mesh")
                .build(),
        )
    }

    fn draw_button(
        &self,
        ctx: &mut Context,
        button: Rect,
        text_str: String,
        el: Element,
    ) -> Button {
        // button outline
        let outline = Mesh::from_data(
            ctx,
            MeshBuilder::new()
                .rectangle(
                    DrawMode::stroke(1f32),
                    Rect::new(0f32, 0f32, button.w, button.h),
                    Color::WHITE,
                )
                .expect("Couldn't draw button")
                .build(),
        );
        // button text
        let mut text = Text::new(TextFragment {
            text: text_str,
            color: Some(Color::WHITE),
            font: self.font.clone(),
            scale: Some(PxScale::from(button.h - 14f32)),
        });
        text.set_bounds(Point2::new(button.w, button.h));
        text.set_layout(TextLayout {
            h_align: TextAlign::Begin,
            v_align: TextAlign::Begin,
        });
        Button {
            frame: outline,
            text: text,
            rect: button,
            el: el,
        }
    }

    fn draw_element_selector(&self, ctx: &mut Context) -> Buttons {
        let mut element_selector: Buttons = vec![];
        // can't enumerate an enum so gotta keep an index separately
        let mut i = 1f32;
        for el in Element::iter() {
            // create button drawables first
            let button_height = 30f32;
            let outline_rect = Rect {
                x: self.frame_element_selector.x,
                y: self.frame_element_selector.y + self.frame_element_selector.h // offset to bottom of selector...
                    - button_height * i // ... account for button height
                    - 10f32 * (i - 1f32), // ... and padding
                w: self.frame_element_selector.w,
                h: button_height,
            };
            let button = self.draw_button(ctx, outline_rect, el.to_string(), el);
            // then add to buttons vec
            element_selector.push(button);
            i += 1f32;
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
        let atoms_m = self.draw_atoms(ctx, state.get_atoms(), self.get_scaling_factor() as i32)?;
        let fps = self.draw_fps(ctx, self.frame_fps, &self.font)?;
        canvas.draw(
            &self.mesh_sandbox.clone().unwrap(),
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
        for button in self.buttons.clone().unwrap().iter() {
            canvas.draw(
                &button.frame,
                DrawParam::default().dest(Point2::new(button.rect.x, button.rect.y)),
            );
            canvas.draw(
                &button.text,
                DrawParam::default().dest(Point2::new(button.rect.x + 7f32, button.rect.y + 7f32)),
            );
        }
        // output drawing
        canvas.finish(ctx)?;
        Ok(())
    }
}
