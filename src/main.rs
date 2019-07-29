#![feature(duration_float)]
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;
extern crate nalgebra;
extern crate nalgebra_glm;
extern crate image;
extern crate rand;
extern crate tobj;
extern crate num;


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
    event::{Event, WindowEvent},
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
use nalgebra_glm::RealField;
use crate::render_gl::math::radians;
use crate::render_gl::camera::Camera;

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
            .build_windowed(output, &event_loop).unwrap();

        let windowed_context = unsafe { windowed_context.make_current().expect("Could not make windowed context current") };

        let context = windowed_context.context();
        let _gl = gl::Gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

        let mut camera = Camera::new(size, &windowed_context.window());

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
        // let rectangle2 = rectangle::Rectangle::new(&res, &gl, nalgebra::Vector2::new(0.5, 0.0))?;

        if let Some(mut context) = self.context.take() {
            let event_loop = context.event_loop.take().unwrap();
            context.camera.set_used(&gl, &cube.program);
            let mut time = Instant::now();
            event_loop.run(move |event, _, control_flow| {
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
                        context.color_buffer.clear(&gl);
                        for pos in &cube_positions {
                            cube.render(&gl, angle + counter as f32, pos);
                            counter += 1;
                        }

                       // rectangle2.render(&gl);
                        context.window.swap_buffers().unwrap();

                    },
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        println!("The close button was pressed; closing");
                        *control_flow = ControlFlow::Exit
                    },
                    _ => *control_flow = ControlFlow::Poll,
                }
            });
        }

        Ok(())
    }
}

