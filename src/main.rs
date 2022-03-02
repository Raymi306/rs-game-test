use engine::run;
mod demos;

fn main() {
    let mut game_state = demos::RandomNoiseLimited::new();
    run(&mut game_state);
}
