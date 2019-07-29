use crate::render_gl::viewport::Viewport;
use crate::render_gl::math::radians;
use nalgebra_glm::TMat4;
use crate::render_gl::Program;
use glutin::{ContextWrapper, dpi::{LogicalSize, PhysicalSize}, window::Window};

pub struct Camera {
    viewport: Viewport,
    size: LogicalSize,
}

impl Camera {
    pub fn new(size: LogicalSize, window: &Window) -> Self {
        let viewport = Viewport::for_window(size, window);
        Camera {
            viewport,
            size,
        }
    }

    pub fn set_used(&self, gl: &gl::Gl, program: &Program) {
        program.set_used();
        self.viewport.set_used(gl);


        let view = nalgebra_glm::identity();
        let view = nalgebra_glm::translate(&view, &nalgebra_glm::vec3(0.0, 0.0, -3.0));

        let projection =  nalgebra_glm::perspective(radians(55.0), self.viewport.aspect_ratio(), 0.1, 100.0);

        program.set_mat_4f("view", view);
        program.set_mat_4f("projection", projection);
    }

    pub fn update_size(&mut self, size: LogicalSize, window: &Window) -> PhysicalSize {
        self.viewport.update_size(size, window)
    }
}