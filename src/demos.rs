extern crate rand;
extern crate sdl2;

use std::time::Duration;

use rand::Rng;
use rand::rngs::ThreadRng;

use engine::{ Context, GameState, timer, Engine };

//pub struct RandomNoise {
//    rng: ThreadRng,
//    ctx: Context,
//}
//
//impl RandomNoise {
//    pub fn new() -> Self {
//        let rng = rand::thread_rng();
//        let ctx = Context {
//            screen_width: 1024,
//            screen_height: 768,
//        };
//        Self {
//            rng,
//            ctx,
//        }
//    }
//}
//
//impl GameState for RandomNoise {
//    fn on_update(&mut self, _elapsed_time: Duration, draw_surface: &mut Surface) {
//        let pb = draw_surface.without_lock_mut().unwrap();
//        for byte in pb {
//             *byte = self.rng.gen();
//        }
//    }
//    fn context(&self) -> &Context {
//        &self.ctx
//    }
//    fn context_mut(&mut self) -> &mut Context {
//        &mut self.ctx
//    }
//}
//
pub struct MixedExample {
    rng: ThreadRng,
    ctx: Context,
    draw_timer: timer::Timer,
}

impl MixedExample {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        let ctx = Context::new(1024, 768);
        Self {
            rng,
            ctx,
            draw_timer: timer::Timer::new(Duration::from_millis(33)),
        }
    }
}

impl GameState for MixedExample {
    fn on_start(&mut self) {
        self.draw_timer.force();
    }
    fn on_update(&mut self,
                 elapsed_time: Duration,
                 ngin: &mut Engine,
                 ) {
        ngin.window.set_title(&format!("Render time: {}ms", elapsed_time.as_millis())).unwrap();
        self.draw_timer.update(elapsed_time);
        let old_keys = ngin.keyboard_state.old_keys();
        let new_keys = ngin.keyboard_state.new_keys();
        if !old_keys.is_empty() || !new_keys.is_empty() {
            println!("old keys: {:?}, new_keys: {:?}", old_keys, new_keys);
        }
        if self.draw_timer.done {
            let pb = ngin.draw_surface.without_lock_mut().unwrap();
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
//
//pub struct RandomRows {
//    rng: ThreadRng,
//    ctx: Context,
//}
//
//impl RandomRows {
//    pub fn new() -> Self {
//        let rng = rand::thread_rng();
//        let ctx = Context {
//            screen_width: 1024,
//            screen_height: 768,
//        };
//        Self {
//            rng,
//            ctx,
//        }
//    }
//}
//
//impl GameState for RandomRows {
//    fn on_update(&mut self, _elapsed_time: Duration, draw_surface: &mut Surface) {
//        let pb = draw_surface.without_lock_mut().unwrap();
//        for i in 0..self.ctx.screen_height {
//            let r = self.rng.gen();
//            let g = self.rng.gen();
//            let b = self.rng.gen();
//            for j in (0..self.ctx.screen_width as usize * 3).step_by(3) {
//                pb[j + (i * self.ctx.screen_width * 3) as usize] = r;
//                pb[j + 1 + (i * self.ctx.screen_width * 3) as usize] = g;
//                pb[j + 2 + (i * self.ctx.screen_width * 3) as usize] = b;
//            }
//        }
//    }
//    fn context(&self) -> &Context {
//        &self.ctx
//    }
//    fn context_mut(&mut self) -> &mut Context {
//        &mut self.ctx
//    }
//}
//
//pub struct RandomCols {
//    rng: ThreadRng,
//    ctx: Context,
//}
//
//impl RandomCols {
//    pub fn new() -> Self {
//        let rng = rand::thread_rng();
//        let ctx = Context {
//            screen_width: 1024,
//            screen_height: 768,
//        };
//        Self {
//            rng,
//            ctx,
//        }
//    }
//}
//
//impl GameState for RandomCols {
//    fn on_update(&mut self, _elapsed_time: Duration, draw_surface: &mut Surface) {
//        let pb = draw_surface.without_lock_mut().unwrap();
//        for x in 0..self.ctx.screen_width {
//            let r = self.rng.gen();
//            let g = self.rng.gen();
//            let b = self.rng.gen();
//            for y in 0..self.ctx.screen_height {
//                pb[((x + self.ctx.screen_width * y) * 3) as usize] = r;
//                pb[((x + self.ctx.screen_width * y) * 3 + 1) as usize] = g;
//                pb[((x + self.ctx.screen_width * y) * 3 + 2) as usize] = b;
//            }
//        }
//    }
//    fn context(&self) -> &Context {
//        &self.ctx
//    }
//    fn context_mut(&mut self) -> &mut Context {
//        &mut self.ctx
//    }
//}
