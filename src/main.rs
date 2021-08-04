use log::{debug, info};
use ggez::*;
mod powder;

fn main() {
    env_logger::init();
    info!("Starting");
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("powder", "hersh")
        .build()
        .expect("Couldn't build the ggez context");
    debug!("Context built");
    debug!("Writing game config");
    filesystem::write_config(&mut ctx, &c).expect("Couldn't write config");
    debug!("Initialising game core");
    let game = powder::Powder::new(&mut ctx).expect("Could not run game core");

    info!("Running event loop");
    event::run(ctx, event_loop, game)
}
