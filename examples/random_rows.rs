extern crate rand;
extern crate sdl2;

use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;

use engine::{run, Context, Engine, GameState};

pub struct RandomRows {
    rng: ThreadRng,
    ctx: Context,
}

impl RandomRows {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        let ctx = Context {
            screen_width: 1024,
            screen_height: 768,
            font_descriptors: None,
        };
        Self { rng, ctx }
    }
}

impl GameState for RandomRows {
    fn on_update(&mut self, _elapsed_time: Duration, ngin: &mut Engine) {
        let pb = ngin.draw_surface.without_lock_mut().unwrap();
        for i in 0..self.ctx.screen_height {
            let r = self.rng.gen();
            let g = self.rng.gen();
            let b = self.rng.gen();
            for j in (0..self.ctx.screen_width as usize * 3).step_by(3) {
                pb[j + (i * self.ctx.screen_width * 3) as usize] = r;
                pb[j + 1 + (i * self.ctx.screen_width * 3) as usize] = g;
                pb[j + 2 + (i * self.ctx.screen_width * 3) as usize] = b;
            }
        }
    }
    fn context(&self) -> &Context {
        &self.ctx
    }
}

fn main() {
    let mut game_state = RandomRows::new();
    run(&mut game_state);
}
