use sdl2::pixels::Color;
use sdl2::surface::Surface;

use crate::types::Vec2;

fn plot(x: i32, y: i32, width: u32, buf: &mut [u8], color: Color) {
    let index = ((x + y * width as i32) * 3) as usize;
    buf[index] = color.r;
    buf[index + 1] = color.g;
    buf[index + 2] = color.b;
}

/// bresenham's algorithm shamelessly stolen from wikipedia's pseudocode
pub fn draw_line(start: Vec2, end: Vec2, surface: &mut Surface, color: Color) {
    let width = surface.width();
    let height = surface.height();
    let pixel_bytes = surface.without_lock_mut().unwrap();
    let distance_x = (end.x - start.x).abs();
    let slope_x;
    if start.x < end.x {
        slope_x = 1;
    } else {
        slope_x = -1;
    }
    let distance_y = -(end.y - start.y).abs();
    let slope_y;
    if start.y < end.y {
        slope_y = 1;
    } else {
        slope_y = -1;
    }
    let mut error = distance_x + distance_y;
    let mut cur_x = start.x;
    let mut cur_y = start.y;
    loop {
        // if offscreen, break if finished otherwise continue
        if cur_x >= width.try_into().unwrap()
            || cur_x < 0
            || cur_y >= height.try_into().unwrap()
            || cur_y < 0
        {
            if cur_x == end.x && cur_y == end.y {
                break;
            }
            continue;
        }
        plot(cur_x, cur_y, width, pixel_bytes, color);
        if cur_x == end.x && cur_y == end.y {
            break;
        }
        let error_2 = 2 * error;
        if error_2 >= distance_y {
            if cur_x == end.x {
                break;
            }
            error = error + distance_y;
            cur_x = cur_x + slope_x;
        }
        if error_2 <= distance_x {
            if cur_y == end.y {
                break;
            }
            error = error + distance_x;
            cur_y = cur_y + slope_y;
        }
    }
}

pub fn draw_triangle(p1: Vec2, p2: Vec2, p3: Vec2, surface: &mut Surface, color: Color) {
    draw_line(p1, p2, surface, color);
    draw_line(p2, p3, surface, color);
    draw_line(p3, p1, surface, color);
}

pub fn draw_vertical_unchecked(p1: Vec2, length: i32, surface: &mut Surface, color: Color) {
    let surface_width = surface.width() as i32;
    let pixel_bytes = surface.without_lock_mut().unwrap();
    for y in p1.y..length + p1.y {
        pixel_bytes[((p1.x + surface_width * y) * 3)     as usize] = color.r;
        pixel_bytes[((p1.x + surface_width * y) * 3 + 1) as usize] = color.g;
        pixel_bytes[((p1.x + surface_width * y) * 3 + 2) as usize] = color.b;
    }
}

pub fn draw_horizontal_unchecked(p1: Vec2, length: i32, surface: &mut Surface, color: Color) {
    let surface_width = surface.width() as i32;
    let pixel_bytes = surface.without_lock_mut().unwrap();
    for x in (p1.x * 3..(length + p1.x) * 3).step_by(3) {
        pixel_bytes[(x +     (p1.y * surface_width * 3)) as usize] = color.r;
        pixel_bytes[(x + 1 + (p1.y * surface_width * 3)) as usize] = color.g;
        pixel_bytes[(x + 2 + (p1.y * surface_width * 3)) as usize] = color.b;
    }
}

pub fn draw_rectangle_unchecked(bottom_left: Vec2, top_right: Vec2, surface: &mut Surface, color: Color) {
    let height = top_right.y - bottom_left.y;
    let width = top_right.x - bottom_left.x;
    draw_vertical_unchecked(bottom_left, height, surface, color);
    draw_horizontal_unchecked(bottom_left, width, surface, color);
    draw_vertical_unchecked(Vec2 { x: top_right.x, y: top_right.y - height }, height, surface, color);
    draw_horizontal_unchecked(Vec2 { x: bottom_left.x, y: top_right.y }, width, surface, color);
}
