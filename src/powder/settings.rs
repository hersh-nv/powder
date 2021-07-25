/*
Store state info that is mutable, but will not be as frequently mutated as object state data.
This includes layout data.
*/

use ggez::graphics::Rect;
use ggez::*;

pub struct Settings {
    pub sandbox_w: u16,
    pub sandbox_h: u16,
    pub frame_sandbox: Rect,
    pub frame_fps: Rect,
}

impl Settings {
    pub fn new(ctx: &mut Context) -> Self {
        let (win_width, win_height) = graphics::drawable_size(ctx);

        let sandbox_w_default: u16 = 512;
        let sandbox_h_default: u16 = 512;

        // calc sandbox frame
        let frame_sandbox = Rect::new(
            (win_width - sandbox_w_default as f32) / 2.0,
            (win_height - sandbox_h_default as f32) / 2.0,
            (sandbox_w_default) as f32,
            (sandbox_h_default) as f32,
        );

        // calc fps frame
        let fps_w = 100f32;
        let fps_h = 20f32;
        let frame_fps = Rect::new(
            frame_sandbox.x + frame_sandbox.w - fps_w,
            frame_sandbox.y - fps_h,
            fps_w,
            fps_h,
        );

        let settings = Settings {
            sandbox_w: sandbox_w_default,
            sandbox_h: sandbox_w_default,
            frame_sandbox: frame_sandbox,
            frame_fps: frame_fps,
        };
        return settings;
    }
}
