use gl::Gl;
use std::time::Instant;
use crate::game::Game;

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
use crate::game_handler::InputEvent::{KeyPressed, KeyReleased, MouseMoved};
use failure::_core::time::Duration;
use std::ops::Add;
use std::thread;
use nalgebra::max;
use num::clamp;

pub const WINDOW_NAME: &str = "Hello Glutin";

pub struct GameContext {
    pub gl: Gl,
    pub window: ContextWrapper<PossiblyCurrent, Window>,
    start_time: Instant,
}

impl GameContext {
    #[allow(dead_code)]
    pub fn dt(&self, time: Instant) -> f64 {
        (Instant::now() - time).as_micros() as f64 / 1_000_000.0
    }
    #[allow(dead_code)]
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    KeyPressed(InputEventData<VirtualKeyCode>),
    KeyReleased(InputEventData<VirtualKeyCode>),
    MouseMoved(f32, f32),
}
#[derive(Debug, Clone, Copy)]
pub struct InputEventData<T> {
    pub data: T,
    pub dt: f64,
}

pub struct GameHandler {
    context: Option<GameContext>,
    event_loop: Option<EventLoop<()>>,
    size: LogicalSize,
}

impl Default for GameHandler {
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

impl GameHandler {
    pub fn new<T: Into<String>>(title: T, size: LogicalSize) -> Result<Self, CreationError> {
        let event_loop = EventLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size);

        let windowed_context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (4, 1)))
            .with_vsync(false)
            .build_windowed(output, &event_loop).unwrap();

        let windowed_context = unsafe { windowed_context.make_current().expect("Could not make windowed context current") };

        let context = windowed_context.context();

        let _gl = gl::Gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);
        unsafe { _gl.Enable(gl::DEPTH_TEST); }

        let game_context = GameContext {
            gl: _gl,
            window: windowed_context,
            start_time: Instant::now(),
        };

        Ok(GameHandler {
            context: Some(game_context),
            event_loop: Some(event_loop),
            size,
        })
    }



    pub fn run<G: Game + 'static>(&mut self) -> Result<(), failure::Error>{
        let mut last_frame = Instant::now();

        let context = self.context.take().unwrap();
        let event_loop = self.event_loop.take().unwrap();
        let mut game = G::new(&context, self.size);
        event_loop.run(move |event, _, control_flow| {
            let now = Instant::now();
            let dt = context.dt(last_frame);
            let delay = clamp((now - last_frame).as_millis() , 0, 8);
            thread::sleep(Duration::from_millis((8 - delay) as u64));
            last_frame = now;
            let mut pending_input = None;

            match event {
                Event::EventsCleared => {
                     context.window.window().request_redraw();
                },
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    let window: &Window = context.window.window();
                    game.resize(size);
                    let size = size.to_physical(window.hidpi_factor());
                    context.window.resize(size);
                },
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    game.render(&context);
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
                    // context.camera.turn(x as f32 * 0.05, y as f32 * 0.05);
                    pending_input = Some(
                        MouseMoved(x as f32, y as f32)
                    );
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
                    // context.camera.zoom(scroll);
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
                        ElementState::Pressed => {pending_input = Some(KeyPressed(
                            InputEventData{
                                data: key,
                                dt,
                            })
                        ); },
                        ElementState::Released => {pending_input = Some(KeyReleased(
                            InputEventData{
                                data: key,
                                dt,
                            })
                        );},
                    }
                }
                _ => *control_flow = ControlFlow::Poll,
            }
            game.update(pending_input, dt as f32, &context);
        });
    }
}

