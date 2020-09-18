extern crate sdl2;

use std::time::Instant;
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
    let mut t1 = Instant::now();
    let mut t2 = Instant::now();
    let mut red: u8 = 55;
    let mut green: u8 = 0;
    let mut blue: u8 = 110;

    'main: loop {
        t1 = Instant::now();
        let elapsed_time = t1.duration_since(t2).as_millis();
        t2 = t1;
        
        red = ((red as u128 + (0.1 * elapsed_time as f64) as u128) % 255) as u8;
        green = ((green as u128 + (0.1 * elapsed_time as f64) as u128) % 255) as u8;
        blue = ((blue as u128 + (0.1 * elapsed_time as f64) as u128) % 255) as u8;
        //green = value_added as u8 % 255;
        redraw(&mut canvas, red, green, blue);
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {}
            }
        }
    }
}

fn redraw(canvas: &mut Canvas<Window>, red: u8, green: u8, blue: u8) {

    canvas.set_draw_color(Color::RGB(0, 100, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(red, green, blue));
    canvas.fill_rect(Rect::new(300, 200, 500, 400));
    canvas.present();
}
