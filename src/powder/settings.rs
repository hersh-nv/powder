use ggez::*;
use ggez::graphics::Rect;

pub struct Settings {
    pub sandbox: Rect,
    pub frame_fps: Rect,
}

impl Settings {
    pub fn new(ctx: &mut Context) -> Self {
        let (width, height) = graphics::drawable_size(ctx);
        let settings = Settings {
            sandbox: Rect::new(
                width / 2.0 - 256.0,
                height / 2.0 - 256.0, 
                512f32, 
                512f32
            ),
            frame_fps: Rect::new(
                width / 2.0 + 256.0 - 100.0,
                height / 2.0 - 256.0 - 20.0,
                100f32,
                30f32,
            )
        };
        println!("frame_fps.y = {}", settings.frame_fps.y);
        return settings;
    }
}
