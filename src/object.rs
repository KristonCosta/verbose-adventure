use console_backend::Color;

#[derive(Debug)]
pub struct Object {
    pub position: (i32, i32),
    pub glyph: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
}

impl Object {
    pub fn new(position: (i32, i32), glyph: char, color: Color, name: &str, blocks: bool) -> Self {
        Object {
            position,
            glyph,
            color,
            blocks,
            name: name.into(),
            alive: true,
        }
    }
}
