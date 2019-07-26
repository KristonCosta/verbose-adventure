#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;
extern crate nalgebra;

pub mod render_gl;
pub mod resources;
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
    pub viewport: Viewport,
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

        let mut viewport = Viewport::for_window(size, &windowed_context.window());
        viewport.set_used(&_gl);

        let color_buffer = ColorBuffer::from_color(nalgebra::Vector3::new(0.3,0.3,0.5));
        color_buffer.set_used(&_gl);


        let render_context = RenderContext {
            viewport,
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

        let triangle = rectangle::Rectangle::new(&res, &gl)?;

        if let Some(mut context) = self.context.take() {
            let event_loop = context.event_loop.take().unwrap();
            event_loop.run(move |event, _, control_flow| {
                match event {
                    Event::EventsCleared => {
                        // window.request_redraw();
                    },
                    Event::WindowEvent {
                        event: WindowEvent::Resized(size),
                        ..
                    } => {
                        let size = context.viewport.update_size(size, &context.window.window());
                        context.viewport.set_used(&gl);
                        context.window.resize(size);
                    },
                    Event::WindowEvent {
                        event: WindowEvent::RedrawRequested,
                        ..
                    } => {
                        context.color_buffer.clear(&gl);
                        triangle.render(&gl);
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
