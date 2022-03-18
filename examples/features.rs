extern crate rand;
extern crate sdl2;

use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::surface::Surface;

use engine::{run, timer, Context, Engine, FontDescriptor, GameState};

///Must contain a Context struct, which contains information for initializing the engine.
///Put any thing the game will need later during its on_update hook here; can be assets, timers,
///variables, etc.
pub struct Features<'a> {
    ctx: Context,
    draw_timer: timer::Timer,
    rng: ThreadRng,
    test_image_surface: Option<Surface<'a>>,
}

///Implement new for a pretty main
impl Features<'_> {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        let path = "resources/fonts/JetbrainsMonoRegular.ttf";
        // must specify before the main loop which fonts you need, if you specify a range of sizes
        // you can scale the resulting surfaces to get other sizes nearby
        let desired_fonts = vec![
            FontDescriptor {
                path,
                label: "font_small",
                size: 12,
            },
            FontDescriptor {
                path,
                label: "font_medium",
                size: 24,
            },
            FontDescriptor {
                path,
                label: "font_large",
                size: 64,
            },
        ];
        let ctx = Context::new(1024, 768, Some(desired_fonts));
        Self {
            ctx,
            rng,
            draw_timer: timer::Timer::new(Duration::from_millis(16)),
            test_image_surface: None,
        }
    }
}

///Implement the 3 key hooks, `on_start`, `on_update`, `on_exit`, as well as the boilerplate function
///`context`. `on_start` and `on_exit` have default implementations if you don't need them and wish
///to just use `on_update`
impl GameState for Features<'_> {
    fn on_start(&mut self, _ngin: &mut Engine) {
        //force the timer to 'done' state
        self.draw_timer.force();
        self.test_image_surface =
            Some(Surface::load_bmp("resources/images/test_pattern_1.bmp").unwrap());
    }
    fn on_update(&mut self, elapsed_time: Duration, ngin: &mut Engine) {
        //ngin provides easy access to sdl internals that you may need direct access to.
        ngin.window
            .set_title(&format!("Render time: {}ms", elapsed_time.as_millis()))
            .unwrap();
        //timers must be updated each tick
        self.draw_timer.update(elapsed_time);
        //keyboard state demonstration
        let old_keys = ngin.keyboard_state.old_keys();
        let new_keys = ngin.keyboard_state.new_keys();
        if !old_keys.is_empty() || !new_keys.is_empty() {
            println!("old keys: {:?}, new_keys: {:?}", old_keys, new_keys);
        }
        //this timer is being used to limit how often the screen draws to 60 fps
        if self.draw_timer.done {
            //different drawing operations:
            //font access
            let font_surface = ngin.fonts()["font_medium"]
                .render(&format!("{}ms", elapsed_time.as_millis()))
                .blended(Color::RGBA(0, 0, 0, 255))
                .unwrap();
            //direct pixel access
            let pb = ngin.draw_surface.without_lock_mut().unwrap();
            for byte in pb {
                *byte = self.rng.gen();
            }
            //image blitting
            self.test_image_surface
                .as_ref()
                .unwrap()
                .blit(None, &mut ngin.draw_surface, None)
                .unwrap();
            font_surface
                .blit(None, &mut ngin.draw_surface, None)
                .unwrap();
            //makes the timer go on forever
            self.draw_timer.restart();
        }
    }
    //boilerplate required to give the game's run function guaranteed access to the context object
    //of the game
    fn context(&self) -> &Context {
        &self.ctx
    }
}

fn main() {
    let mut game_state = Features::new();
    run(&mut game_state);
}
