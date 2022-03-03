extern crate sdl2;

use std::collections::VecDeque;
use std::convert::TryInto;
use std::time::{ Instant, Duration };

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

pub enum Event {
    SetWindowTitle(String),
}

pub trait GameState {
    fn on_start(&mut self, _event_queue: &mut VecDeque<Event>) {}
    fn on_update(
        &mut self,
        elapsed_time: Duration,
        draw_surface: &mut Surface,
        event_queue: &mut VecDeque<Event>
        );
    fn on_exit(&mut self, _event_queue: &mut VecDeque<Event>) {}
    fn context(&self) -> &Context;
    fn context_mut(&mut self) -> &mut Context;
}

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
    let mut pixel_buffer = vec![100_u8; (win_x * win_y * 3).try_into().expect("Somehow overflowed a usize with num screen pixels")].into_boxed_slice();
    let mut draw_surface = Surface::from_data(&mut pixel_buffer, win_x, win_y, 3 * win_x, PixelFormatEnum::RGB24).unwrap();
    let mut t1 = Instant::now();
    let mut events = VecDeque::<Event>::new();
    {
        game_state.on_start(&mut events);
    }
    'main: loop {
        let elapsed_time = t1.elapsed();
        t1 = Instant::now();
        for event in events.drain(..).collect::<VecDeque<Event>>() {
            match event {
                Event::SetWindowTitle(s) => window.set_title(s.as_str()).unwrap(),
            };
        }
        {
            game_state.on_update(elapsed_time, &mut draw_surface, &mut events);
        }
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
        game_state.on_exit(&mut events);
    }
}
