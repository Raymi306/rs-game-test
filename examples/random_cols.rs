extern crate rand;
extern crate sdl2;

use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;
use sdl2::pixels::Color;

use engine::{run, Context, Engine, GameState};
use engine::types::Vec2;
use engine::drawing::draw_vertical_unchecked;

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
        for x in 0..self.ctx.screen_width {
            let r = self.rng.gen();
            let g = self.rng.gen();
            let b = self.rng.gen();
            draw_vertical_unchecked(
                Vec2 {x: x.try_into().unwrap(), y: 0},
                self.ctx.screen_height as i32,
                &mut ngin.draw_surface,
                Color { r, g, b, a: 255 }
                );
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
