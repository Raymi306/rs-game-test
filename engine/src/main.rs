extern crate rand;
extern crate sdl2;

use rand::Rng;
use rand::rngs::ThreadRng;
use sdl2::surface::Surface;

use engine::{ Context, GameState, run };

struct NoiseMaker<'a> {
    rng: &'a mut ThreadRng,
    ctx: Context,
}

impl GameState for NoiseMaker<'_> {
    fn on_update(&mut self, _elapsed_time: u128, draw_surface: &mut Surface) {
        let pb = draw_surface.without_lock_mut().unwrap();
        for i in 0..pb.len() {
             pb[i] = self.rng.gen();
        }
    }
    fn context(&self) -> &Context {
        &self.ctx
    }
    fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }
}

struct RandomRows<'a> {
    rng: &'a mut ThreadRng,
    ctx: Context,
}

impl GameState for RandomRows<'_> {
    fn on_update(&mut self, _elapsed_time: u128, draw_surface: &mut Surface) {
        let pb = draw_surface.without_lock_mut().unwrap();
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
    fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }
}

struct RandomCols<'a> {
    rng: &'a mut ThreadRng,
    ctx: Context,
}

impl GameState for RandomCols<'_> {
    fn on_update(&mut self, _elapsed_time: u128, draw_surface: &mut Surface) {
        let pb = draw_surface.without_lock_mut().unwrap();
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
    fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }
}

fn main() {
    let ctx = Context {
        screen_width: 1024,
        screen_height: 768,
    };
    let mut rng = rand::thread_rng();
    let mut game_state = RandomRows {
        rng: &mut rng,
        ctx: ctx,
    };
    run(&mut game_state);
}
