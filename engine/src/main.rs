extern crate rand;
extern crate sdl2;

use rand::Rng;
use rand::rngs::ThreadRng;

use sdl2::surface::Surface;

use engine::{ GameState, run };

struct NoiseMaker {
    rng: ThreadRng,
}

impl GameState for NoiseMaker {
    fn on_start(&mut self) {}
    fn on_exit(&mut self) {}
    fn on_update(&mut self, draw_surface: &mut Surface, _elapsed_time: u128) {
        let pb = draw_surface.without_lock_mut().unwrap();
        for i in 0..pb.len() {
             pb[i] = self.rng.gen();
        }
    }
}

struct RandomRows {
    rng: ThreadRng,
}

impl GameState for RandomRows {
    fn on_start(&mut self) {}
    fn on_exit(&mut self) {}
    fn on_update(&mut self, draw_surface: &mut Surface, _elapsed_time: u128) {
        let pb = draw_surface.without_lock_mut().unwrap();
        for i in 0..768 {
            let r = self.rng.gen();
            let g = self.rng.gen();
            let b = self.rng.gen();
            for j in (0..1024 * 3).step_by(3) {
                pb[j + (i * 1024 * 3)] = r;
                pb[j + 1 + (i * 1024 * 3)] = g;
                pb[j + 2 + (i * 1024 * 3)] = b;
            }
        }
    }
}

struct RandomCols {
    rng: ThreadRng,
}

impl GameState for RandomCols {
    fn on_start(&mut self) {}
    fn on_exit(&mut self) {}
    fn on_update(&mut self, draw_surface: &mut Surface, _elapsed_time: u128) {
        let pb = draw_surface.without_lock_mut().unwrap();
        for x in 0..1024 {
            let r = self.rng.gen();
            let g = self.rng.gen();
            let b = self.rng.gen();
            for y in 0..768 {
                pb[(x + 1024 * y) * 3] = r;
                pb[(x + 1024 * y) * 3 + 1] = g;
                pb[(x + 1024 * y) * 3 + 2] = b;
            }
        }
    }
}

fn main() {
    let mut game_state = RandomCols { rng: rand::thread_rng() };
    run(1024, 768, &mut game_state);
}
