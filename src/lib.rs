extern crate sdl2;

use std::collections::{ VecDeque, HashSet };
use std::convert::TryInto;
use std::time::{ Instant, Duration };

use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

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

/// Represents possible interactions with sdl2 objects, in addition to abstract events
pub enum Event {
    SetWindowTitle(String),
}

pub trait GameState {
    fn on_start(&mut self, _event_queue: &mut VecDeque<Event>) {}
    fn on_update(
        &mut self,
        elapsed_time: Duration,
        keyboard_state: &KeyboardState,
        draw_surface: &mut Surface,
        event_queue: &mut VecDeque<Event>
        );
    fn on_exit(&mut self, _event_queue: &mut VecDeque<Event>) {}
    fn context(&self) -> &Context;
    fn context_mut(&mut self) -> &mut Context;
}

/// handles boilerplate sdl2 instantiatiations and the main loop
/// Within the main loop, manages an event queue for interacting with sdl2
/// objects safely, calls GameState hooks, and blits to the screen after 
/// on_update. Finally, this function manages the sdl event pump
pub fn run<T: GameState>(game_state: &mut T) {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let ctx = game_state.context();
    let win_x = ctx.screen_width;
    let win_y = ctx.screen_height;
    let mut window = video_subsystem
        .window("Game", win_x, win_y)
        .build()
        .unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut prev_keys = HashSet::new();
    let mut pixel_buffer = vec![100_u8; (win_x * win_y * 3).try_into().expect("Somehow overflowed a usize with num screen pixels")].into_boxed_slice();
    let mut draw_surface = Surface::from_data(&mut pixel_buffer, win_x, win_y, 3 * win_x, PixelFormatEnum::RGB24).unwrap();
    let mut t1 = Instant::now();
    let mut event_queue = VecDeque::<Event>::new();
    {
        game_state.on_start(&mut event_queue);
    }
    'main: loop {
        let elapsed_time = t1.elapsed();
        t1 = Instant::now();
        for event in event_queue.drain(..).collect::<VecDeque<Event>>() {
            match event {
                Event::SetWindowTitle(s) => window.set_title(s.as_str()).unwrap(),
            }
        }
        let cur_keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        let keyboard_state = KeyboardState {
            previous: prev_keys,
            current: cur_keys,
        };
        {
            game_state.on_update(elapsed_time, &keyboard_state, &mut draw_surface, &mut event_queue);
        }
        prev_keys = keyboard_state.current;
        let mut window_surface = window.surface(&event_pump).unwrap();
        window_surface.update_window().unwrap();
        draw_surface.blit(None, &mut window_surface, None).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {}
            }
        }
    }
    {
        game_state.on_exit(&mut event_queue);
    }
}
