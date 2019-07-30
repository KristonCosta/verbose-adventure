#![feature(duration_float)]
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;
extern crate nalgebra;
extern crate nalgebra_glm;
extern crate image;
extern crate rand;
extern crate tobj;
extern crate num;

use num::Float;
pub mod render_gl;
pub mod resources;
mod cube;
mod triangle;
mod rectangle;
mod debug;

use gl::Gl;
use render_gl::{data, color_buffer};
use glutin::{
    ContextBuilder,
    ContextWrapper,
    PossiblyCurrent,
    CreationError,
    dpi::LogicalSize,
    window::{Window, WindowBuilder},
    event::{Event, WindowEvent, DeviceEvent, KeyboardInput, VirtualKeyCode, ElementState, MouseScrollDelta},
    event_loop::{EventLoop, ControlFlow},
    GlRequest,
    Api};
use resources::Resources;
use std::path::Path;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray};
use crate::debug::failure_to_string;
use crate::render_gl::viewport::Viewport;
use crate::render_gl::color_buffer::ColorBuffer;
use nalgebra::Matrix4;
use std::time::Instant;
use nalgebra_glm::{RealField, Vec3, U3};
use crate::render_gl::math::radians;
use crate::render_gl::camera::Camera;
use failure::_core::time::Duration;
use std::collections::HashMap;

pub const WINDOW_NAME: &str = "Hello Glutin";

pub fn main() {
    let mut glutin_state = GlutinState::default();
    if let Err(e) = glutin_state.run() {
        println!("{}", failure_to_string(e));
    }
}

pub struct RenderContext {
    pub event_loop: Option<EventLoop<()>>,
    pub window: ContextWrapper<PossiblyCurrent, Window>,
    pub camera: Camera,
    pub color_buffer: ColorBuffer,
}

pub struct GlutinState {
    context: Option<RenderContext>,
    gl: Gl,
}

impl Default for GlutinState {
    fn default() -> Self {
        Self::new(
            WINDOW_NAME,
            LogicalSize{
                width: 800.0,
                height: 600.0,
            },
        ).expect("Could not create a window!")
    }
}

impl GlutinState {
    pub fn new<T: Into<String>>(title: T, size: LogicalSize) -> Result<Self, CreationError> {
        let event_loop = EventLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size);

        let windowed_context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (4, 1)))
            .with_vsync(true)
            .build_windowed(output, &event_loop).unwrap();

        let windowed_context = unsafe { windowed_context.make_current().expect("Could not make windowed context current") };

        let context = windowed_context.context();

        let _gl = gl::Gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

        let mut camera = Camera::new(size, Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 0.0, 0.0), &windowed_context.window());

        let window: &Window = windowed_context.window();
        window.set_cursor_grab(true);
        let color_buffer = ColorBuffer::from_color(nalgebra::Vector3::new(0.3,0.3,0.5));
        color_buffer.set_used(&_gl);


        let render_context = RenderContext {
            camera,
            event_loop: Some(event_loop),
            window: windowed_context,
            color_buffer,
        };

        Ok(GlutinState {
            context: Some(render_context),
            gl: _gl,
        })
    }



    pub fn run(&mut self) -> Result<(), failure::Error>{
        let gl = self.gl.clone();
        let res = Resources::from_relative_exe_path(Path::new("assets"))?;
        unsafe { gl.Enable(gl::DEPTH_TEST); }

        // let rectangle = rectangle::Rectangle::new(&res, &gl, nalgebra::Vector2::new(0.0, 0.0))?;
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
        let cube = cube::Cube::new(&res, &gl, nalgebra::Vector3::new(0.0,0.0,0.0))?;

        if let Some(mut context) = self.context.take() {
            let event_loop = context.event_loop.take().unwrap();
            context.camera.set_used(&gl, &cube.program);
            let mut time = Instant::now();
            let mut last_frame = Instant::now();
            let mut input_fps = Instant::now();
            let mut render_fps = Instant::now();
            let mut camera_fps = Instant::now();
            context.window.window().request_redraw();
            let mut input_map: HashMap<VirtualKeyCode, bool> = [
                (VirtualKeyCode::A, false),
                (VirtualKeyCode::D, false),
                (VirtualKeyCode::W, false),
                (VirtualKeyCode::S, false),
            ].iter().cloned().collect();

            event_loop.run(move |event, _, control_flow| {
                let now = Instant::now();
                let dt = ((now - last_frame).as_micros() as f64 / 1_000_000.0);
                last_frame = now;
                match event {
                    Event::EventsCleared => {
                        context.window.window().request_redraw();
                    },
                    Event::WindowEvent {
                        event: WindowEvent::Resized(size),
                        ..
                    } => {
                        let size = context.camera.update_size(size, &context.window.window());
                        context.camera.set_used(&gl, &cube.program);
                        context.window.resize(size);
                    },
                    Event::WindowEvent {
                        event: WindowEvent::RedrawRequested,
                        ..
                    } => {
                        let elapsed = time.elapsed().as_secs_f32() * 1.0;
                        let mut angle = elapsed * radians(50.0);
                        let mut counter = 1;
                        let camera_position = Vec3::new(elapsed.sin() * 10.0, 0.0, elapsed.cos() * 10.0);
                        context.camera.set_used(&gl, &cube.program);
                        // context.camera.reset_mouse_position(&context.window.window());
                        context.color_buffer.clear(&gl);
                        for pos in &cube_positions {
                            cube.render(&gl, angle + counter as f32, pos);
                            counter += 1;
                        }
                        context.window.swap_buffers().unwrap();

                    },
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        println!("The close button was pressed; closing");
                        *control_flow = ControlFlow::Exit
                    },

                    Event::DeviceEvent {
                        event: DeviceEvent::MouseMotion {
                            delta: (x, y),
                        },
                        ..
                    } => {
                        context.camera.turn(x as f32 * 0.05, y as f32 * 0.05);
                    },
                    Event::WindowEvent {
                        event: WindowEvent::MouseWheel {
                            delta,
                            ..
                        },
                        ..
                    } => {
                        let scroll =  match delta {
                            MouseScrollDelta::LineDelta(_, y) => {
                                y
                            },
                            MouseScrollDelta::PixelDelta(pos) => {
                                pos.y as f32
                            },
                        };
                        println!("Zoom: {}", scroll);
                        context.camera.zoom(scroll);
                    },
                    Event::WindowEvent {
                        event: WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                virtual_keycode: Some(key),
                                state,
                                ..
                            },
                            ..
                        },
                        ..
                    } => {
                        if key == VirtualKeyCode::Escape {
                            *control_flow = ControlFlow::Exit
                        }
                        match state {
                            ElementState::Pressed => {input_map.insert(key, true);},
                            ElementState::Released => {input_map.insert(key, false);},
                        }
                    }
                    _ => *control_flow = ControlFlow::Poll,
                }

                let front = context.camera.front();
                let up = context.camera.up();
                let speed = 1.0 * dt as f32;// * dt as f32;


                let norm_cross = front.cross(&up).normalize() * speed;
                if input_map.contains_key(&VirtualKeyCode::W) && input_map[&VirtualKeyCode::W] {
                    context.camera.translate_position((front * speed));
                }
                if input_map.contains_key(&VirtualKeyCode::A) && input_map[&VirtualKeyCode::A] {
                    context.camera.translate_position((norm_cross) * -1 as f32);
                }
                if input_map.contains_key(&VirtualKeyCode::S) && input_map[&VirtualKeyCode::S] {
                    context.camera.translate_position((front * speed * -1 as f32));
                }
                if input_map.contains_key(&VirtualKeyCode::D) && input_map[&VirtualKeyCode::D] {
                    context.camera.translate_position((norm_cross));
                }
                if input_map.contains_key(&VirtualKeyCode::Z) && input_map[&VirtualKeyCode::Z] {
                    context.camera.zoom(0.5);
                }
                if input_map.contains_key(&VirtualKeyCode::X) && input_map[&VirtualKeyCode::X] {
                    context.camera.zoom(-0.5);
                }
            });
        }

        Ok(())
    }
}

