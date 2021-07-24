use ggez::*;

mod renderer;

pub struct Powder {
    dt: std::time::Duration,
}

impl Powder {
    pub fn new() -> Self {
        Powder { dt: std::time::Duration::new(0, 0) }
    }
}

impl ggez::event::EventHandler<GameError> for Powder {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics_engine::draw(ctx);
        println!("Hello ggez! dt = {}ns", self.dt.as_nanos());
        Ok(())
    }
}