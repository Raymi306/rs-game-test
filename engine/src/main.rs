extern crate sdl2;

use std::time::Instant;
use std::convert::TryFrom;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .build()
        .unwrap();

    let mut canvas : Canvas<Window> = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    let instant = Instant::now();
    let mut green: u8 = 0;

    'main: loop {
        let elapsed_time = instant.elapsed().as_millis();
        let new_color = green + (0.1 * elapsed_time as f64) as u8 % 255;
        //green = value_added as u8 % 255;
        redraw(&mut canvas, new_color);
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {}
            }
        }
    }
}

fn redraw(canvas: &mut Canvas<Window>, green: u8) {

    canvas.set_draw_color(Color::RGB(0, 100, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(15, green, 120));
    canvas.fill_rect(Rect::new(300, 200, 500, 400));
    canvas.present();
}
