use crate::resources::Resources;
use font_renderer::{load_bitmap, BoundingBox};
use crate::render_gl;
use image::GenericImageView;
use crate::render_gl::texture::Texture;
use crate::render_gl::buffer::{VertexArray, ArrayBuffer, ElementArrayBuffer};
use std::collections::HashMap;
use crate::render_gl::{Program, data};
use failure::_core::cell::RefCell;
use crate::console_vertex::Vertex;
use crate::glyph::Glyph;
use glutin::{
    dpi::LogicalSize,
};
use core::ptr;

pub struct Console {
    is_dirty: RefCell<Dirty>,
    num_vert: RefCell<Num>,
    vao: VertexArray,
    vbo: ArrayBuffer,
    ebo: ElementArrayBuffer,
    texture: Texture,
    glyph_size: (f32, f32),
    glyph_map: HashMap<char, BoundingBox>,
    glyphs: HashMap<u32, Glyph>,
    program: Program,
    texture_scale: (i32, i32),
    dimensions: (u32, u32),
    screen_size: LogicalSize,
    default_background: data::f32_f32_f32_f32,
}

struct Num(i32);

impl Num {
    pub fn set(&mut self, v: i32) {
        self.0 = v;
    }
}

struct Dirty(bool);

impl Dirty {
    pub fn set(&mut self, v: bool) {
        self.0 = v;
    }
}

impl Console {
    pub fn new(res: &Resources, gl: &gl::Gl, size: (u32, u32), screen_size: LogicalSize, background: data::f32_f32_f32_f32) -> Result<Self, failure::Error> {
        let shader_program = render_gl::Program::from_res(
            &gl, &res, "shaders/glyph",
        )?;
        let font_bytes = res.load_bytes_from_file("droid-sans-mono.ttf").unwrap();
        let (font_img, glyph_map, _) = load_bitmap(font_bytes);
        let texture_scale_u32 = font_img.dimensions();
        let texture = Texture::from_img(gl, font_img, gl::RGBA)?;
        let glyph_size = (2.0 * screen_size.width as f32 / size.0 as f32, 2.0 * screen_size.height as f32 / size.1 as f32);
        let texture_scale = (texture_scale_u32.0 as i32, texture_scale_u32.1 as i32);

        let vao = VertexArray::new(&gl);
        let vbo = ArrayBuffer::new(&gl);
        let ebo = ElementArrayBuffer::new(&gl);

        Ok(Console {
            is_dirty: RefCell::new(Dirty(true)),
            num_vert: RefCell::new(Num(0)),
            vao,
            vbo,
            ebo,
            texture,
            glyph_map,
            glyph_size,
            texture_scale,
            glyphs: HashMap::new(),
            program: shader_program,
            dimensions: size,
            screen_size,
            default_background: background
        })
    }

    pub fn clear(&mut self) {
        self.glyphs.clear();
        self.is_dirty.borrow_mut().set(true);
    }

    pub fn put_char(&mut self, c: char, x: i32, y: i32, foreground: data::f32_f32_f32_f32, background: Option<data::f32_f32_f32_f32>, layer: u32) {
        if x < 0 || y < 0 {
            return
        }
        let background = match background {
            Some(b) => b,
            None => self.default_background
        };
        self.is_dirty.borrow_mut().set(true);
        self.glyphs.insert(self.coordinates_to_index(x as u32, y as u32), Glyph::new(c, background, foreground, layer));
    }

    fn coordinates_to_index(&self, x: u32, y: u32) -> u32 {
        x + y * self.dimensions.0
    }

    fn index_to_coordinates(&self, index: u32) -> (u32, u32) {
        (index % self.dimensions.0, index / self.dimensions.0)
    }

    fn coordinates_to_fractional(&self, coordinates: (u32, u32)) -> (f32, f32) {
        ((coordinates.0 as f32 / self.dimensions.0 as f32) * 2.0 - 1.0, (coordinates.1 as f32 / self.dimensions.1 as f32) * 2.0 - 1.0)
    }

    fn bound_box_to_fractional(&self, coordinates: (f32, f32)) -> (f32, f32) {
        ((coordinates.0 as f32 / self.screen_size.width as f32), (coordinates.1 as f32 / self.screen_size.height as f32))
    }

    fn load_gl(&self, gl: &gl::Gl) -> i32 {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<gl::types::GLuint> = vec![];

        let mut num_glyphs = 0;
        for (index, glyph) in self.glyphs.iter() {
            let bounding_box = self.glyph_map.get(&glyph.character).unwrap();
            let scaled_bounding_box = self.bound_box_to_fractional(self.glyph_size);
            let coordinates = self.coordinates_to_fractional(self.index_to_coordinates(*index));
            let index_offset = vertices.len() as u32;
            vertices.append(&mut vec![
                Vertex { position: (scaled_bounding_box.0 + coordinates.0, scaled_bounding_box.1 + coordinates.1, glyph.layer as f32 / 255.0 * -1.0).into(),
                    texture: bounding_box.top_right(self.texture_scale).into(),
                    foreground: glyph.foreground,
                    background: glyph.background},
                Vertex { position: (scaled_bounding_box.0 + coordinates.0, coordinates.1, glyph.layer as f32 / 255.0  * -1.0 ).into(),
                    texture: bounding_box.bottom_right(self.texture_scale).into(),
                    foreground: glyph.foreground,
                    background: glyph.background },
                Vertex { position: (coordinates.0, coordinates.1, glyph.layer as f32 / 255.0 * -1.0).into(),
                    texture: bounding_box.bottom_left(self.texture_scale).into(),
                    foreground: glyph.foreground,
                    background: glyph.background },
                Vertex { position: (coordinates.0, scaled_bounding_box.1 + coordinates.1,glyph.layer as f32 / 255.0 * -1.0).into(),
                    texture: bounding_box.top_left(self.texture_scale).into(),
                    foreground: glyph.foreground,
                    background: glyph.background },
            ]);
            indices.append(&mut vec![
                index_offset, 1 + index_offset, 3 + index_offset, 1 + index_offset, 2 + index_offset, 3 + index_offset,
            ]);
            num_glyphs += 1;
        }

        self.vao.bind();

        self.vbo.bind();
        self.vbo.dynamic_draw_data(&vertices);

        self.ebo.bind();
        self.ebo.dynamic_draw_data(&indices);

        Vertex::vertex_attrib_pointers(&gl);

        self.vbo.unbind();
        self.vao.unbind();
        self.ebo.unbind();
        num_glyphs
    }

    pub fn render(&self, gl: &gl::Gl) {

        if self.is_dirty.borrow().0 {
            let num_glyphs = self.load_gl(&gl);
            self.is_dirty.borrow_mut().set(false);
            self.num_vert.borrow_mut().set(num_glyphs);
        }
        self.program.set_used();
        unsafe {
            gl.Disable(gl::DEPTH_TEST);
        }
        self.texture.bind();

        self.vao.bind();
        unsafe {
            gl.DrawElements(
                gl::TRIANGLES,
                self.num_vert.borrow().0 * 6,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}
