extern crate sdl2;

use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

use engine::{run, timer, Context, Engine, FontDescriptor, GameState};

///Must contain a Context struct, which contains information for initializing the engine.
///Put any thing the game will need later during its on_update hook here; can be assets, timers,
///variables, etc.
pub struct Features {
    ctx: Context,
    draw_timer: timer::Timer,
}

///Implement new for a pretty main
impl Features {
    pub fn new() -> Self {
        let path = "resources/fonts/JetbrainsMonoRegular.ttf";
        // must specify before the main loop which fonts you need, if you specify a range of sizes
        // you can scale the resulting surfaces to get other sizes nearby
        let desired_fonts = vec![
            FontDescriptor {
                path,
                label: "font_medium",
                size: 24,
            },
        ];
        let ctx = Context::new(1024, 768, Some(desired_fonts));
        Self {
            ctx,
            draw_timer: timer::Timer::new(Duration::from_millis(16)),
        }
    }
}

///Implement the 3 key hooks, `on_start`, `on_update`, `on_exit`, as well as the boilerplate function
///`context`. `on_start` and `on_exit` have default implementations if you don't need them and wish
///to just use `on_update`
impl GameState for Features {
    fn on_start(&mut self, _ngin: &mut Engine) {
        //force the timer to 'done' state
        self.draw_timer.force();
    }
    fn on_update(&mut self, elapsed_time: Duration, ngin: &mut Engine) {
        //ngin provides easy access to sdl internals that you may need direct access to.
        ngin.canvas
            .window_mut()
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
            ngin.create_font_texture(
                "font",
                &ngin.fonts["font_medium"],
                &format!("{}ms", elapsed_time.as_millis()),
                Color::RGBA(0, 0, 0, 255)
                );

            //primitives
            let p1 = Point::new(0, 0);
            let p2 = Point::new(1024, 768);
            ngin.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
            ngin.canvas.draw_line(p1, p2).unwrap();
            ngin.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
            let rect = Rect::new(100, 100, 200, 400);
            ngin.canvas.draw_rect(rect).unwrap();
            //image blitting
            //ngin.canvas.copy(&self.image_texture.as_ref().unwrap(), None, None).unwrap();
            //ngin.canvas.copy(&font_texture, None, None).unwrap();
            
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
