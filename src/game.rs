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
    keyboard: HashMap<VirtualKeyCode, bool>,
}

impl Game for GameImpl  {
    fn new(context: &GameContext, size: LogicalSize) -> Self {
        let window: &Window = context.window.window();
        window.set_cursor_grab(true);
        let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
        let font_bytes = res.load_bytes_from_file("droid-sans-mono.ttf").unwrap();

        let physical_size = size.to_physical(window.hidpi_factor());
        context.window.resize(physical_size);

        let color_buffer = ColorBuffer::from_color(nalgebra::Vector3::new(0.3,0.3,0.5));
        color_buffer.set_used(&context.gl);

        let camera = Camera::new(size, Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 0.0, 0.0), &window);
        let cube = Cube::new(&res, &context.gl, nalgebra::Vector3::new(0.0,0.0,0.0)).unwrap();

        let mut input_map: HashMap<VirtualKeyCode, bool> = [].iter().cloned().collect();

        GameImpl {
            color_buffer,
            camera,
            cube,
            keyboard:input_map,
        }
    }

    fn render(&self, context: &GameContext) {
        let gl = &context.gl;
        let cube_positions = vec![
            nalgebra::Vector3::new(0.0,0.0,0.0),
            nalgebra::Vector3::new(2.0,5.0,-15.0),
            nalgebra::Vector3::new(-1.5,-2.2,2.5),
            nalgebra::Vector3::new(-3.8,-2.0,-12.3),
            nalgebra::Vector3::new(2.4,-0.4,-3.5),
            nalgebra::Vector3::new(-1.7,3.0,-7.5),
            nalgebra::Vector3::new(1.3,-2.0,-2.5),
            nalgebra::Vector3::new(1.5,2.0,-2.5),
            nalgebra::Vector3::new(1.5,0.2,-1.5),
            nalgebra::Vector3::new(-1.3,1.0,-1.5),
        ];
        let elapsed = context.elapsed().as_secs_f32();

        let mut angle = (50.0.to_radians() * elapsed) as f32;
        let mut counter = 1;
        let camera_position = Vec3::new(elapsed.sin() * 10.0, 0.0, elapsed.cos() * 10.0);
        self.camera.set_used(&gl, &self.cube.program);
        self.color_buffer.clear(&gl);
        for pos in &cube_positions {
            self.cube.render(&gl, counter as f32 + angle, pos);
            counter += 1;
        }
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