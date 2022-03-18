extern crate rand;
extern crate sdl2;

use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;

use engine::{run, Context, Engine, GameState};

pub struct RandomCols {
    rng: ThreadRng,
    ctx: Context,
}

impl RandomCols {
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

impl GameState for RandomCols {
    fn on_update(&mut self, _elapsed_time: Duration, ngin: &mut Engine) {
        let pb = ngin.draw_surface.without_lock_mut().unwrap();
        for x in 0..self.ctx.screen_width {
            let r = self.rng.gen();
            let g = self.rng.gen();
            let b = self.rng.gen();
            for y in 0..self.ctx.screen_height {
                pb[((x + self.ctx.screen_width * y) * 3) as usize] = r;
                pb[((x + self.ctx.screen_width * y) * 3 + 1) as usize] = g;
                pb[((x + self.ctx.screen_width * y) * 3 + 2) as usize] = b;
            }
        }
    }
    fn context(&self) -> &Context {
        &self.ctx
    }
}

fn main() {
    let mut game_state = RandomCols::new();
    run(&mut game_state);
}
