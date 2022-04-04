extern crate rand;
extern crate sdl2;

use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;
use sdl2::pixels::Color;

use engine::{run, Context, Engine, GameState};
use engine::types::Vec2;
use engine::drawing::draw_horizontal_unchecked;

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
        for y in 0..self.ctx.screen_height {
            let r = self.rng.gen();
            let g = self.rng.gen();
            let b = self.rng.gen();
            draw_horizontal_unchecked(
                Vec2 { x: 0, y: y.try_into().unwrap()},
                self.ctx.screen_width as i32,
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
    let mut game_state = RandomRows::new();
    run(&mut game_state);
}
