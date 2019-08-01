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
use num::Float;
use std::collections::HashMap;
use font_renderer::load_bitmap;
use crate::glyph::Glyph;
use crate::console::Console;

pub trait Game {
    fn new(context: &GameContext, size: LogicalSize) -> Self;
    fn render(&self, context: &GameContext);
    fn update(&mut self, dt: f32, context: &GameContext);
    fn process_input(&mut self, pending_input: InputEvent, context: &GameContext);
}

pub struct GameImpl {
    color_buffer: ColorBuffer,
    camera: Camera,
    cube: Cube,
    console: Console,
    keyboard: HashMap<VirtualKeyCode, bool>,
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
        let cube = Cube::new(&res, &context.gl, nalgebra::Vector3::new(0.0,0.0,0.0)).unwrap();
        // let glyph = Glyph::new(&res, &context.gl,nalgebra::Vector3::new(0.0,0.0,0.0), 'p' ).unwrap();
        let mut console = Console::new(&res, &context.gl,(50, 30), size.clone()).unwrap();
        let mut input_map: HashMap<VirtualKeyCode, bool> = [].iter().cloned().collect();

        let mut index = 0;
        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            console.put_char(ch, index, 0);
            index += 1;
        }

        console.put_char('b', 3, 4);
        console.put_char('c', 6, 4);
        console.put_char('?', 8, 8);
        console.put_char('<', 1, 9);

        GameImpl {
            color_buffer,
            camera,
            cube,
            console,
            keyboard:input_map,
        }
    }

    fn render(&self, context: &GameContext) {
        let gl = &context.gl;
        self.color_buffer.clear(&gl);
        self.console.render(&gl);
    }

    fn update(&mut self, dt: f32, context: &GameContext) {
        let front = self.camera.front();
        let up = self.camera.up();
        let speed = 1.0 * dt as f32;// * dt as f32;

        let norm_cross = front.cross(&up).normalize() * speed;
        let input_map = &self.keyboard;
        if input_map.contains_key(&VirtualKeyCode::W) && input_map[&VirtualKeyCode::W] {
            self.camera.translate_position((front * speed));
        }
        if input_map.contains_key(&VirtualKeyCode::A) && input_map[&VirtualKeyCode::A] {
            self.camera.translate_position((norm_cross) * -1 as f32);
        }
        if input_map.contains_key(&VirtualKeyCode::S) && input_map[&VirtualKeyCode::S] {
            self.camera.translate_position((front * speed * -1 as f32));
        }
        if input_map.contains_key(&VirtualKeyCode::D) && input_map[&VirtualKeyCode::D] {
            self.camera.translate_position((norm_cross));
        }
        if input_map.contains_key(&VirtualKeyCode::Z) && input_map[&VirtualKeyCode::Z] {
            self.camera.zoom(0.5);
        }
        if input_map.contains_key(&VirtualKeyCode::X) && input_map[&VirtualKeyCode::X] {
            self.camera.zoom(-0.5);
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