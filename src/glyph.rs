use crate::render_gl;
use crate::render_gl::{buffer, data};
use crate::resources::Resources;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray, ElementArrayBuffer};
use crate::render_gl::texture::Texture;
use nalgebra_glm::{TVec, TVec1, RealField};
use crate::render_gl::math::radians;
use nalgebra::{Vector3, Vector4};
use font_renderer::load_bitmap;
use image::GenericImageView;

#[derive(Debug, Copy, Clone)]
pub struct Glyph {
    pub character: char,
    pub background: data::f32_f32_f32,
}

impl Glyph {
    pub fn new(character: char, background:  data::f32_f32_f32) -> Self {
        Glyph{
            character,
            background,
        }
    }
}
