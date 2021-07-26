/*
Store state info that is mutable, but will not be as frequently mutated as object state data.
Cache whatever data here you like.
This includes layout data.
*/

use ggez::graphics::Rect;
use ggez::*;

#[derive(Debug)]
pub struct SettingsError;

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Couldn't make this settings change")
    }
}

pub struct Settings {
    pub sandbox_w: u16,
    pub sandbox_h: u16,
    pub frame_sandbox: Rect,
    pub frame_fps: Rect,

    pub scaling_factor: u8,

    // TODO
    pub mesh_sandbox: Option<graphics::Mesh>,
}

impl Settings {
    pub fn new(ctx: &mut Context) -> Self {
        const DEF_SANDBOX_W: u16 = 512;
        const DEF_SANDBOX_H: u16 = 512;
        const DEF_SCALING_FACTOR: u8 = 10;

        let (win_width, win_height) = graphics::drawable_size(ctx);

        // calc sandbox frame
        let frame_sandbox = Rect::new(
            (win_width - DEF_SANDBOX_W as f32) / 2.0,
            (win_height - DEF_SANDBOX_H as f32) / 2.0,
            (DEF_SANDBOX_W) as f32,
            (DEF_SANDBOX_H) as f32,
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

        Settings {
            sandbox_w: DEF_SANDBOX_W,
            sandbox_h: DEF_SANDBOX_W,
            frame_sandbox: frame_sandbox,
            frame_fps: frame_fps,
            scaling_factor: DEF_SCALING_FACTOR,
            mesh_sandbox: None,
        }
    }

    pub fn set_scaling_factor(&mut self, sf: u8) -> Result<(), SettingsError> {
        match sf {
            1..=10 => {
                self.scaling_factor = sf;
                Ok(())
            }
            _ => Err(SettingsError),
        }
    }

    pub fn get_scaling_factor(&self) -> u8 {
        self.scaling_factor
    }
}
