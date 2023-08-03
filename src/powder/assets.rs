use ggez::{graphics::FontData, Context, GameResult};
use log::warn;

pub struct Assets {
    pub font: Option<String>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let font_path = String::from("/font/Roboto Mono M-PL.ttf");
        Ok(Assets {
            font: match FontData::from_path(ctx, font_path) {
                Ok(font) => Some(font_path),
                Err(_) => {
                    warn!("Failed to load font! Using default");
                    None
                }
            },
        })
    }
}
