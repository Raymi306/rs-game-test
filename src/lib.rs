extern crate sdl2;

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::{Duration, Instant};

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

pub mod timer;
pub mod types;

pub struct FontDescriptor {
    pub path: &'static str,
    pub label: &'static str,
    pub size: u16,
}

pub struct Context {
    pub screen_width: u32,
    pub screen_height: u32,
    pub font_descriptors: Option<Vec<FontDescriptor>>,
}

impl Context {
    pub fn new(
        screen_width: u32,
        screen_height: u32,
        font_descriptors: Option<Vec<FontDescriptor>>,
    ) -> Self {
        Self {
            screen_width,
            screen_height,
            font_descriptors,
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

pub struct Engine<'a, 'b> {
    pub keyboard_state: KeyboardState,
    pub canvas: Canvas<Window>,
    pub fonts: HashMap<String, Font<'a, 'b>>,
    pub _texture_creator: TextureCreator<WindowContext>,
    pub textures: HashMap<String, Texture<'a>>,
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn create_font_texture(&'a mut self, name: &str, font: &Font, text: &str, color: Color) {
        let font_surface = font.render(text).blended(color).unwrap();
        self.create_texture_from_surface(name, font_surface);
    }
    pub fn create_texture_from_surface(&'a mut self, name: &str, surface: Surface) {
        let texture = self._texture_creator.create_texture_from_surface(surface).unwrap();
        self.textures.insert(name.to_string(), texture);
    }
}

pub trait GameState {
    fn on_start<E: Engine>(&mut self, _ngin: &mut E) {}
    fn on_update<E: Engine>(&mut self, elapsed_time: Duration, ngin: &mut E);
    fn on_exit(&mut self) {}
    fn context(&self) -> &Context;
    //fn context_mut(&mut self) -> &mut Context;
}

fn load_fonts<'a, 'b>(
    ttf_context: &'a Sdl2TtfContext,
    font_descriptors: &[FontDescriptor],
) -> HashMap<String, Font<'a, 'b>> {
    let mut result: HashMap<String, Font> = HashMap::new();
    for descriptor in font_descriptors {
        let path = Path::new(descriptor.path);
        let font = ttf_context.load_font(path, descriptor.size).unwrap();
        result.insert(descriptor.label.to_string(), font);
    }
    result
}

/// Handles boilerplate sdl2 instantiatiations and the main loop.
/// Within the main loop, calls GameState hooks, and blits to the screen after
/// on_update. Finally, this function manages the sdl event pump.
pub fn run<T: GameState>(game_state: &mut T) {
    let sdl: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let ctx = game_state.context();
    let mut event_pump = sdl.event_pump().unwrap();
    let window = video_subsystem
        .window("Game", ctx.screen_width, ctx.screen_height)
        .build()
        .unwrap();
    let canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .unwrap();
    let mut t1 = Instant::now();
    let keyboard_state = KeyboardState {
        previous: HashSet::new(),
        current: HashSet::new(),
    };
    let fonts = ctx
        .font_descriptors
        .as_ref()
        .map(|inner| load_fonts(&ttf_context, inner))
        .unwrap();
    let _texture_creator = canvas.texture_creator();
    let textures = HashMap::new();
    let mut ngin = Engine {
        keyboard_state,
        canvas,
        fonts,
        _texture_creator,
        textures,
    };

    game_state.on_start(&mut ngin);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        let elapsed_time = t1.elapsed();
        t1 = Instant::now();
        ngin.keyboard_state.current = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        game_state.on_update(elapsed_time, &mut ngin);
        ngin.keyboard_state.previous = ngin.keyboard_state.current;
        ngin.canvas.present();
    }
    game_state.on_exit();
}
