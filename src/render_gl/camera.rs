use crate::render_gl::viewport::Viewport;
use crate::render_gl::math::radians;
use nalgebra_glm::{TMat4, Mat4, Vec3, U3};
use crate::render_gl::Program;
use glutin::{ContextWrapper, dpi::{LogicalSize, PhysicalSize}, window::Window};
use nalgebra::min;
use num::clamp;
use num::Float;

pub struct Camera {
    viewport: Viewport,
    position: Vec3,
    target: Vec3,
    pitch: f32,
    yaw: f32,
    fov: f32,
    pub size: LogicalSize,
}

impl Camera {
    pub fn new(size: LogicalSize, position: Vec3, target: Vec3, window: &Window) -> Self {
        let viewport = Viewport::for_window(size, window);
        Camera {
            viewport,
            size,
            position,
            target,
            pitch: 0.0,
            yaw: -90.0,
            fov: 55.0,
        }
    }

    pub fn translate_position(&mut self, translate: Vec3) {
        self.position += translate;
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }

    pub fn direction(&self) -> Vec3 {
        nalgebra_glm::normalize(&(self.position - self.target))
    }

    pub fn right(&self) -> Vec3 {
        nalgebra_glm::cross::<f32, U3>(&Vec3::new(0.0, 1.0, 0.0), &self.direction())
    }

    pub fn up(&self) -> Vec3 {
        nalgebra_glm::cross::<f32, U3>(&self.direction(), &self.right())
    }

    pub fn front(&self) -> Vec3 {
        let front_x = self.pitch.to_radians().cos() * self.yaw.to_radians().cos();
        let front_y = self.pitch.to_radians().sin();
        let front_z = self.pitch.to_radians().cos() * self.yaw.to_radians().sin();
        nalgebra_glm::normalize(&Vec3::new(front_x, front_y, front_z))
    }

    pub fn look_at(&self) -> Mat4 {
        nalgebra_glm::look_at(
            &self.position,
            &(self.position + self.front()),
            &self.up(),
        )
    }

    pub fn set_used(&self, gl: &gl::Gl, program: &Program) {
        program.set_used();
        self.viewport.set_used(gl);

        let view = self.look_at();
        let projection =  nalgebra_glm::perspective(radians(self.fov), self.viewport.aspect_ratio(), 0.1, 100.0);

        program.set_mat_4f("view", view);
        program.set_mat_4f("projection", projection);
    }

    pub fn update_size(&mut self, size: LogicalSize, window: &Window) -> PhysicalSize {
        self.viewport.update_size(size, window)
    }

    pub fn reset_mouse_position(&self, window: &Window) -> () {
        window.set_cursor_position((self.size.width / 2.0, self.size.height/ 2.0).into()).unwrap();
    }

    pub fn zoom(&mut self, dy: f32) {
        println!("Zoom: {}", dy);
        self.fov = clamp(self.fov + dy * -1.0, 1.0, 80.0);
    }

    pub fn turn(&mut self, dx:f32, dy:f32) {
        self.yaw += dx;
        self.pitch = clamp(self.pitch + dy * -1.0, -90.0, 90.0);
    }
}