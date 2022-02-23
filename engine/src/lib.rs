extern crate sdl2;

use std::time::Instant;
use std::convert::TryInto;

use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;


pub trait GameState {
    fn on_start(&mut self);
    fn on_update(&mut self, draw_surface: &mut Surface, elapsed_time: u128);
    fn on_exit(&mut self);
}

pub fn run(win_x: u32, win_y: u32, game_state: &mut dyn GameState) {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", win_x, win_y)
        .build()
        .unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut pixel_buffer = vec![100 as u8; (win_x * win_y * 3).try_into().expect("Somehow overflowed a usize with num screen pixels")].into_boxed_slice();
    let mut draw_surface = Surface::from_data(&mut pixel_buffer, win_x, win_y, 3 * win_x, PixelFormatEnum::RGB24).unwrap();
    let mut t1;
    let mut t2 = Instant::now();
    game_state.on_start();
    'main: loop {
        t1 = Instant::now();
        let elapsed_time = t1.duration_since(t2).as_millis();
        t2 = t1;
        {
            game_state.on_update(&mut draw_surface, elapsed_time);
            let mut window_surface = window.surface(&mut event_pump).unwrap();
            window_surface.update_window().unwrap();
            draw_surface.blit(None, &mut window_surface, None).unwrap();
        }
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {}
            }
        }
    }
    game_state.on_exit();
}
