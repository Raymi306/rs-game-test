extern crate rand;
extern crate sdl2;

use std::time::Duration;

use rand::Rng;
use rand::rngs::ThreadRng;
use sdl2::surface::Surface;

use engine::{ Context, GameState, run, timer };

struct RandomNoise {
    rng: ThreadRng,
    ctx: Context,
}

impl RandomNoise {
    fn new() -> Self {
        let rng = rand::thread_rng();
        let ctx = Context {
            screen_width: 1024,
            screen_height: 768,
        };
        Self {
            rng,
            ctx,
        }
    }
}

impl GameState for RandomNoise {
    fn on_update(&mut self, _elapsed_time: Duration, draw_surface: &mut Surface) {
        let pb = draw_surface.without_lock_mut().unwrap();
        for byte in pb {
             *byte = self.rng.gen();
        }
    }
    fn context(&self) -> &Context {
        &self.ctx
    }
    fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }
}

struct RandomNoiseLimited {
    rng: ThreadRng,
    ctx: Context,
    draw_timer: timer::Timer,
}

impl RandomNoiseLimited {
    fn new() -> Self {
        let rng = rand::thread_rng();
        let ctx = Context {
            screen_width: 1024,
            screen_height: 768,
        };
        Self {
            rng,
            ctx,
            draw_timer: timer::Timer::new(Duration::from_millis(100)),
        }
    }
}

impl GameState for RandomNoiseLimited {
    fn on_start(&mut self) {
        self.draw_timer.force();
    }
    fn on_update(&mut self, elapsed_time: Duration, draw_surface: &mut Surface) {
        self.draw_timer.update(elapsed_time);
        if self.draw_timer.done {
            let pb = draw_surface.without_lock_mut().unwrap();
            for byte in pb {
                 *byte = self.rng.gen();
            }
            self.draw_timer.restart();
        }
    }
    fn context(&self) -> &Context {
        &self.ctx
    }
    fn context_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }
}

struct RandomRows {
    rng: ThreadRng,
    ctx: Context,
}

impl RandomRows {
    fn new() -> Self {
        let rng = rand::thread_rng();
        let ctx = Context {
            screen_width: 1024,
            screen_height: 768,
        };
        Self {
            rng,
            ctx,
        }
    }
}

impl GameState for RandomRows {
    fn on_update(&mut self, _elapsed_time: Duration, draw_surface: &mut Surface) {
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

struct RandomCols {
    rng: ThreadRng,
    ctx: Context,
}

impl RandomCols {
    fn new() -> Self {
        let rng = rand::thread_rng();
        let ctx = Context {
            screen_width: 1024,
            screen_height: 768,
        };
        Self {
            rng,
            ctx,
        }
    }
}

impl GameState for RandomCols {
    fn on_update(&mut self, _elapsed_time: Duration, draw_surface: &mut Surface) {
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
    let mut game_state = RandomNoiseLimited::new();
    run(&mut game_state);
}
