use ggez::*;

pub struct Settings {
    pub sandbox: graphics::Rect,
}

impl Settings {
    pub fn new(ctx: &mut Context) -> Self {
        let (width, height) = graphics::drawable_size(ctx);
        Settings {
            sandbox: graphics::Rect::new(
                width/2.0 - 256.0,
                height/2.0 - 256.0,
                512f32,
                512f32,
            ),
        }
    }

}
