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

#[derive(Clone)]
pub struct Settings {
    pub sandbox_w: i32,
    pub sandbox_h: i32,
}

impl Settings {
    pub fn new(sandbox_size: i32) -> Self {
        let sandbox_w = sandbox_size;
        let sandbox_h = sandbox_size;
        // for now things are gonna break in the renderer if this isn't square
        // so let's just mandate that for now
        assert_eq!(sandbox_w, sandbox_h);

        Settings {
            sandbox_w: sandbox_w,
            sandbox_h: sandbox_h,
        }
    }
}
