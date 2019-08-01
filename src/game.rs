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
use crate::color::Color;
use crate::map::{Map, make_map};
use crate::object::Object;

pub trait Game {
    fn new(context: &GameContext, size: LogicalSize) -> Self;
    fn render(&mut self, context: &GameContext);
    fn update(&mut self, dt: f32, context: &GameContext);
    fn process_input(&mut self, pending_input: InputEvent, context: &GameContext);
}


pub struct GameImpl {
    color_buffer: ColorBuffer,
    camera: Camera,
    console: Console,
    map: Map,
    player: Object,
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
        let mut console = Console::new(&res, &context.gl,map_size, size.clone(), data::f32_f32_f32_f32::new(255.0, 255.0, 255.0, 1.0)).unwrap();
        let mut input_map: HashMap<VirtualKeyCode, bool> = [].iter().cloned().collect();

        let (map, player_pos) = make_map(map_size.0 as usize, map_size.1 as usize, 15, 5, 20);
        let color_dark_ground: Color = Color::new(50.0,50.0, 150.0, 1.0);
        let player = Object::new(player_pos, '@', color_dark_ground);
        GameImpl {
            color_buffer,
            camera,
            console,
            player,
            map,
            keyboard:input_map,
            input_limiter: Instant::now(),
        }
    }

    fn render(&mut self, context: &GameContext) {
        let gl = &context.gl;
        let color_dark_wall: Color = Color::from_int(0, 0, 100, 1.0);
        let color_dark_ground: Color = Color::from_int(50, 50, 150, 1.0);
        let clear: Color = Color::from_int(0, 0, 0, 0.0);
        let white: Color = Color::from_int(200, 200,200,1.0);
        self.color_buffer.clear(&gl);
        self.console.clear();
        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                if !(self.map[x][y].block_sight) {
                    self.console.put_char(
                        ' ',
                        x as i32, y as i32, clear, Some(color_dark_ground));

                } else {
                    self.console.put_char(
                        ' ',
                        x as i32, y as i32, clear, Some(color_dark_wall))
                }
            }
        }
        self.console.put_char('@', self.player.position.0, self.player.position.1, white, Some(clear));
        self.console.render(&gl);
    }

    fn update(&mut self, dt: f32, context: &GameContext) {
        if self.input_limiter.elapsed() > Duration::from_millis(1) {
            let front = self.camera.front();
            let up = self.camera.up();
            let speed = 1.0 * dt as f32;// * dt as f32;

            let norm_cross = front.cross(&up).normalize() * speed;
            let input_map = &self.keyboard;
            if input_map.contains_key(&VirtualKeyCode::W) && input_map[&VirtualKeyCode::W] {
                self.player.move_by(0, 1, &self.map);
            }
            if input_map.contains_key(&VirtualKeyCode::A) && input_map[&VirtualKeyCode::A] {
                self.player.move_by(-1, 0, &self.map);
            }
            if input_map.contains_key(&VirtualKeyCode::S) && input_map[&VirtualKeyCode::S] {
                self.player.move_by(0, -1, &self.map);
            }
            if input_map.contains_key(&VirtualKeyCode::D) && input_map[&VirtualKeyCode::D] {
                self.player.move_by(1, 0, &self.map);
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