
pub struct RenderContext {
    pub gl: Gl,
    pub window: ContextWrapper<PossiblyCurrent, Window>,
}

pub struct GameHandler {
    context: Option<RenderContext>,
    event_loop: Option<EventLoop<()>>,

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
            .with_vsync(true)
            .build_windowed(output, &event_loop).unwrap();

        let windowed_context = unsafe { windowed_context.make_current().expect("Could not make windowed context current") };

        let context = windowed_context.context();

        let _gl = gl::Gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);
        unsafe { _gl.Enable(gl::DEPTH_TEST); }

        let render_context = RenderContext {
            gl: _gl,
            window: windowed_context,
        };

        Ok(GameHandler {
            context: Some(render_context),
            event_loop: Some(event_loop),

        })
    }



    pub fn run(&mut self) -> Result<(), failure::Error>{

        let mut time = Instant::now();
        let mut last_frame = Instant::now();
        let mut input_map: HashMap<VirtualKeyCode, bool> = [].iter().cloned().collect();
        let context = self.context.take().unwrap();
        let gl = context.gl.clone();
        let event_loop = self.event_loop.take().unwrap();
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
                    let window: &Window = context.window.window();
                    let size = size.to_physical(window.hidpi_factor());
                    context.window.resize(size);
                },
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
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
                        ElementState::Pressed => {input_map.insert(key, true);},
                        ElementState::Released => {input_map.insert(key, false);},
                    }
                }
                _ => *control_flow = ControlFlow::Poll,
            }
        });

        Ok(())
    }
}

