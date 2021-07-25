use ggez::*;
mod powder;

fn main() {
    println!("Starting");
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("powder", "hersh")
        .build()
        .expect("Couldn't build the ggez context");
    println!("Context built");
    println!("Writing game config");
    filesystem::write_config(&mut ctx, &c).expect("Couldn't write config");
    println!("Running game core");
    let game = powder::Powder::new(&mut ctx).expect("Could not run game core");

    println!("Running event loop");
    event::run(ctx, event_loop, game)
}
