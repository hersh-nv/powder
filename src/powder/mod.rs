use ggez::*;
use log::debug;

mod assets;
mod event_handles;
mod renderer;
mod state;

use assets::Assets;
use state::State;

pub struct Powder {
    state: state::State,
    assets: Assets,
}

impl Powder {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut powder = Powder {
            state: State::new(),
            assets: Assets::new(ctx)?,
        };
        powder.init(ctx)?;
        Ok(powder)
    }

    fn init(&mut self, _ctx: &mut Context) -> GameResult {
        debug!("Core init");
        self.state.init();
        Ok(())
    }
}

impl ggez::event::EventHandler<GameError> for Powder {
    /* Required methods for EventHandler trait */
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        event_handles::update(ctx, &mut self.state).ok();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        renderer::draw(ctx, &self.state, &self.assets)?;
        timer::yield_now();
        Ok(())
    }

    /* Optional methods, event handlers */
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        event_handles::mouse_button_down_event(ctx, &mut self.state, button, x, y)
    }
}
