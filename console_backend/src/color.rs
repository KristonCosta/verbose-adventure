use crate::render_gl::data;

pub mod colors {
    use crate::Color;
    lazy_static! {
        pub static ref RED: Color = Color::from_int(1,2,3,4.4);
        pub static ref DARK_RED: Color = Color::from_int(130, 20, 20, 1.0);
        pub static ref DARK_GREEN: Color = Color::from_int(20, 120, 20, 1.0);
        pub static ref NAVY: Color = Color::from_int(0, 0, 100, 1.0);
        pub static ref DARK_INDIGO: Color = Color::from_int(50, 50, 150, 1.0);
        pub static ref CLEAR: Color = Color::from_int(0, 0, 0, 0.0);
        pub static ref BLACK: Color = Color::from_int(0, 0, 0, 1.0);
        pub static ref WHITE: Color = Color::from_int(255, 255, 255, 1.0);
        pub static ref DESATURATED_GREEN: Color = Color::from_int(25, 125, 35, 1.0);
        pub static ref LIGHT_GREEN: Color = Color::from_int(90, 165, 30, 1.0);
        pub static ref LIGHT_SLATE_BLUE: Color = Color::from_int(160, 65, 255, 1.0);
    }
}

pub type Color = data::f32_f32_f32_f32;

impl Color {
    pub fn from_int(x: i32, y: i32, z: i32, a: f32) -> Color {
        data::f32_f32_f32_f32::new(x as f32 / 255.0, y as f32 / 255.0, z as f32 / 255.0, a as f32)
    }
}