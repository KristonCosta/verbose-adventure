use crate::render_gl;
use crate::render_gl::{buffer, data};
use crate::resources::Resources;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray, ElementArrayBuffer};
use crate::render_gl::texture::Texture;
use nalgebra_glm::{TVec, TVec1, RealField};
use crate::render_gl::math::radians;
use nalgebra::Vector3;
use font_renderer::load_bitmap;
use image::GenericImageView;

#[derive(Debug, Copy, Clone)]
pub struct Glyph {
    pub character: char,
}

impl Glyph {
    pub fn new(character: char) -> Self {
        Glyph{
            character,
        }
    }
}
