use ggez::*;
mod powder;

fn main() {
    println!("Starting");
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "hersh")
        .default_conf(c)
        .build()
        .expect("Couldn't build the ggez context");
    println!("Context built");
    println!("Running game core");
    let game = powder::Powder::new(&mut ctx).expect("Could not run game core");

    println!("Running event loop");
    event::run(ctx, event_loop, game)
}
