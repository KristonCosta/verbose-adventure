use crate::render_gl::data;

pub type Color = data::f32_f32_f32;

impl Color {
    pub fn from_int(x: i32, y: i32, z: i32) -> Color {
        data::f32_f32_f32::new(x as f32 / 255.0, y as f32 / 255.0, z as f32 / 255.0)
    }
}