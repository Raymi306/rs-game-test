extern crate sdl2;

use std::cell::RefCell;
use std::collections::{ HashSet };
use std::convert::TryInto;
use std::time::{ Instant, Duration };

use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use sdl2::video::Window;

pub mod timer;

pub struct Context {
    pub screen_width: u32,
    pub screen_height: u32,
}

impl Context {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            screen_width,
            screen_height,
        }
    }
}

//TODO controls, mouse + keyboard
//TODO control mapping
//TODO create controls module
pub struct KeyboardState {
    pub previous: HashSet<Keycode>,
    pub current: HashSet<Keycode>,
}

impl KeyboardState {
    pub fn new_keys(&self) -> HashSet<Keycode> {
        &self.current - &self.previous
    }
    pub fn old_keys(&self) -> HashSet<Keycode> {
        &self.previous - &self.current
    }
}

pub struct Engine<'a> {
    pub keyboard_state: KeyboardState,
    pub window: RefCell<Window>,
    pub draw_surface: RefCell<Surface<'a>>,
}

pub trait GameState {
    fn on_start(&mut self) {}
    fn on_update(
        &mut self,
        elapsed_time: Duration,
        ngin: &mut Engine,
        );
    fn on_exit(&mut self) {}
    fn context(&self) -> &Context;
    fn context_mut(&mut self) -> &mut Context;
}

/// handles boilerplate sdl2 instantiatiations and the main loop.
/// Within the main loop, manages an event queue for interacting with sdl2
/// objects safely, calls GameState hooks, and blits to the screen after 
/// on_update. Finally, this function manages the sdl event pump.
pub fn run<T: GameState>(game_state: &mut T) {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let ctx = game_state.context();
    let win_x = ctx.screen_width;
    let win_y = ctx.screen_height;
    let window = RefCell::new(video_subsystem
        .window("Game", win_x, win_y)
        .build()
        .unwrap());
    let mut event_pump = sdl.event_pump().unwrap();
    let prev_keys = HashSet::new();
    let mut pixel_buffer = vec![100_u8; (win_x * win_y * 3).try_into().expect("Somehow overflowed a usize with num screen pixels")].into_boxed_slice();
    let draw_surface = RefCell::new(Surface::from_data(&mut pixel_buffer, win_x, win_y, 3 * win_x, PixelFormatEnum::RGB24).unwrap());
    let mut t1 = Instant::now();


    let keyboard_state = KeyboardState {
        previous: prev_keys,
        current: event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect(),
    };

    let mut ngin = Engine {
        keyboard_state,
        window,
        draw_surface,
    };

    game_state.on_start();
    'main: loop {
        let elapsed_time = t1.elapsed();
        t1 = Instant::now();
        ngin.keyboard_state.current = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        game_state.on_update(elapsed_time, &mut ngin);
        ngin.keyboard_state.previous = ngin.keyboard_state.current;
        let mut window_surface = ngin.window.get_mut().surface(&event_pump).unwrap();
        window_surface.update_window().unwrap();
        ngin.draw_surface.get_mut().blit(None, &mut window_surface, None).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {}
            }
        }
    }
    game_state.on_exit();
}
