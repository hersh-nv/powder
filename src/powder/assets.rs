use ggez::{graphics::Font, Context, GameResult};

pub struct Assets {
    pub font: Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Assets {
            font: match Font::new(ctx, "/font/Roboto Mono M-PL.ttf") {
                Ok(font) => font,
                Err(_) => {
                    println!("Failed to load font! Using default");
                    Font::default()
                }
            },
        })
    }
}
