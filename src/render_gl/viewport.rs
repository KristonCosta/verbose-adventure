use gl;
use glutin::{ContextWrapper, dpi::{LogicalSize, PhysicalSize}, window::Window};

pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Viewport {
    pub fn for_window(size: LogicalSize, window: &Window) -> Self {
        let physical_size = get_physical_size(size, window);
        Viewport {
            x: 0,
            y: 0,
            w: physical_size.width as i32,
            h: physical_size.height as i32,
        }
    }

    pub fn update_size(&mut self, size: LogicalSize, window: &Window) -> PhysicalSize {
        let physical_size = get_physical_size(size, window);
        self.w = physical_size.width as i32;
        self.h = physical_size.height as i32;
        physical_size
    }

    pub fn set_used(&self, gl: &gl::Gl) {
        unsafe {
            gl.Viewport(self.x, self.y, self.w, self.h);
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        (self.w as f32) / (self.h as f32)
    }
}

fn get_physical_size(size: LogicalSize, window: &Window) -> PhysicalSize {
    size.to_physical(window.hidpi_factor())
}