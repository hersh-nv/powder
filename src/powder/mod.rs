use ggez::*;

mod assets;
mod settings;
mod state;
mod renderer;

use assets::Assets;
use settings::Settings;
use state::State;

pub struct Powder {
    dt: std::time::Duration,
    state: state::State,
    settings: Settings,
    assets: Assets,
}

impl Powder {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut powder = Powder {
            dt: std::time::Duration::new(0, 0),
            state: State::new(),
            settings: Settings::new(ctx),
            assets: Assets::new(ctx)?,
        };
        powder.init(ctx)?;
        Ok(powder)
    }

    fn init(&mut self, ctx: &mut Context) -> GameResult {
        println!("Core init");
        self.state.init();
        Ok(())
    }
}

impl ggez::event::EventHandler<GameError> for Powder {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        renderer::draw(ctx, &self.settings, &self.state, &self.assets)?;
        timer::yield_now();
        Ok(())
    }
}
