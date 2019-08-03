use crate::render_gl::{data};

#[derive(Debug, Copy, Clone)]
pub struct Glyph {
    pub character: char,
    pub background: data::f32_f32_f32_f32,
    pub foreground: data::f32_f32_f32_f32,
}

impl Glyph {
    pub fn new(character: char, background:  data::f32_f32_f32_f32, foreground: data::f32_f32_f32_f32) -> Self {
        Glyph{
            character,
            background,
            foreground,
        }
    }
}
