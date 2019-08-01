use crate::game_handler::{GameContext, InputEvent};
use crate::render_gl::color_buffer::ColorBuffer;
use crate::render_gl::camera::Camera;
use glutin::{
    dpi::LogicalSize,
    window::Window,
    event::VirtualKeyCode,
};
use failure::Fail;
use nalgebra_glm::{RealField, Vec3, U3};
use crate::cube::Cube;
use crate::resources::Resources;
use std::path::Path;
use num::{Float, clamp};
use std::collections::HashMap;
use font_renderer::load_bitmap;
use crate::glyph::Glyph;
use crate::console::Console;
use std::time::Instant;
use failure::_core::time::Duration;
use crate::render_gl::data;

pub trait Game {
    fn new(context: &GameContext, size: LogicalSize) -> Self;
    fn render(&mut self, context: &GameContext);
    fn update(&mut self, dt: f32, context: &GameContext);
    fn process_input(&mut self, pending_input: InputEvent, context: &GameContext);
}

type Color = data::f32_f32_f32;

const COLOR_DARK_WALL: data::f32_f32_f32 = Color::new(0.0,0.0, 100.0);
const COLOR_DARK_GROUND: data::f32_f32_f32 = Color::new(50.0,50.0, 150.0);

pub struct Object {
    position: (i32, i32),
    ch: char,
    color: Color,
}

impl Object {
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position.0 = clamp(x, 0, self.clamp.0 - 1);
        self.position.1 = clamp(y, 0, self.clamp.1 - 1);
    }

    pub fn dx(&mut self, dx: i32) {
        self.set_position(self.position.0 + dx, self.position.1);
    }

    pub fn dy(&mut self, dy: i32) {
        self.set_position(self.position.0, self.position.1 + dy);
    }
}


pub struct GameImpl {
    color_buffer: ColorBuffer,
    camera: Camera,
    console: Console,
    player: Player,
    keyboard: HashMap<VirtualKeyCode, bool>,
    input_limiter: Instant,
}

impl Game for GameImpl  {
    fn new(context: &GameContext, size: LogicalSize) -> Self {
        let window: &Window = context.window.window();
        let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

        let physical_size = size.to_physical(window.hidpi_factor());
        context.window.resize(physical_size);

        let color_buffer = ColorBuffer::from_color(nalgebra::Vector3::new(0.3,0.3,0.5));
        color_buffer.set_used(&context.gl);

        let camera = Camera::new(size, Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 0.0, 0.0), &window);
        let map_size = (80, 50);
        let mut console = Console::new(&res, &context.gl,map_size, size.clone(), data::f32_f32_f32::new(0.0, 0.0, 0.0)).unwrap();
        let mut input_map: HashMap<VirtualKeyCode, bool> = [].iter().cloned().collect();
        let player = Player {
            position: (25, 15),
            clamp: (map_size.0 as i32, map_size.1 as i32),
        };
        GameImpl {
            color_buffer,
            camera,
            console,
            player,
            keyboard:input_map,
            input_limiter: Instant::now(),
        }
    }

    fn render(&mut self, context: &GameContext) {
        let gl = &context.gl;
        self.color_buffer.clear(&gl);
        self.console.clear();
        self.console.put_char('@', self.player.position.0, self.player.position.1, Some(data::f32_f32_f32::new(100.0,0.0,0.0)));
        self.console.put_char(' ', self.player.position.0 + 1, self.player.position.1 + 1, Some(data::f32_f32_f32::new(100.0,100.0,0.0)));
        self.console.render(&gl);
    }

    fn update(&mut self, dt: f32, context: &GameContext) {
        if self.input_limiter.elapsed() > Duration::from_millis(50) {
            let front = self.camera.front();
            let up = self.camera.up();
            let speed = 1.0 * dt as f32;// * dt as f32;

            let norm_cross = front.cross(&up).normalize() * speed;
            let input_map = &self.keyboard;
            if input_map.contains_key(&VirtualKeyCode::W) && input_map[&VirtualKeyCode::W] {
                self.player.dy(1);
            }
            if input_map.contains_key(&VirtualKeyCode::A) && input_map[&VirtualKeyCode::A] {
                self.player.dx(-1);
            }
            if input_map.contains_key(&VirtualKeyCode::S) && input_map[&VirtualKeyCode::S] {
                self.player.dy(-1);
            }
            if input_map.contains_key(&VirtualKeyCode::D) && input_map[&VirtualKeyCode::D] {
                self.player.dx(1);
            }
            self.input_limiter = Instant::now();
        }

    }

    fn process_input(&mut self, pending_input: InputEvent, context: &GameContext) {
        match pending_input {
            InputEvent::KeyReleased(event) => {
                self.keyboard.insert(event.data, false);
            },
            InputEvent::KeyPressed(event) => {
                self.keyboard.insert(event.data, true);
            },
            InputEvent::MouseMoved(dx, dy) => {
                self.camera.turn(dx as f32 * 0.05, dy as f32 * 0.05);
            },
        }
    }
}