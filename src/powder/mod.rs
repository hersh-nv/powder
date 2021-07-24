use ggez::*;

mod renderer;
mod state;

pub struct Powder {
    dt: std::time::Duration,
    state: state::State,
}

impl Powder {
    pub fn new(ctx: &mut Context) -> Self {
        let mut powder = Powder { 
            dt: std::time::Duration::new(0, 0),
            state: state::State::new(),
        };
        powder.init(ctx);
        return powder
    }

    fn init(&mut self, ctx: &mut Context) {
        println!("Core init");
        self.state.init();
    }
}

impl ggez::event::EventHandler<GameError> for Powder {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Calling renderer");
        renderer::draw(ctx, &self.state)?;
        // println!("Hello ggez! dt = {}ns", self.dt.as_nanos());
        Ok(())
    }
}