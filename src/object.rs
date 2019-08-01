use crate::map::{is_valid_move, Map};
use nalgebra::Vector;
use crate::color::Color;

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

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        let desired_x = self.position.0 + dx;
        let desired_y = self.position.1 + dy;
        if is_valid_move(map, desired_x, desired_y) {
            self.position.0 = desired_x;
            self.position.1 = desired_y;
        }
    }
}
