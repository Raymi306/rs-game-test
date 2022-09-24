extern crate rand;
extern crate sdl2;

use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Point;

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
        for y in 0..self.ctx.screen_height {
            let r = self.rng.gen();
            let g = self.rng.gen();
            let b = self.rng.gen();
            ngin.canvas.set_draw_color(Color { r, g, b, a: 255 });
            ngin.canvas.draw_line(
                Point::new(0, y.try_into().unwrap()),
                Point::new(self.ctx.screen_width as i32, y.try_into().unwrap())
                ).unwrap();
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
