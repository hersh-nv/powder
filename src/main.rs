use ggez::*;
mod powder;

fn main() {
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "hersh")
    .default_conf(c)
    .build()
    .expect("Couldn't build the ggez context");
    let game = powder::Powder::new(&mut ctx);
    
    event::run(ctx, event_loop, game)
}
