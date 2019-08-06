use crate::resources::Resources;
use font_renderer::{load_bitmap, BoundingBox};
use crate::{render_gl, Color};
use image::{GenericImageView, DynamicImage};
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
use crate::color::colors;
use nalgebra_glm::scale;

pub enum Transformer {
    AspectRatio(f32, f32),
}

impl Transformer {
    pub fn apply(&self, console: &mut Console) -> &Self {
        match self {
            Transformer::AspectRatio(desired, actual ) => Transformer::apply_aspect_ratio(console, *desired, *actual)
        }
        self
    }

    fn apply_aspect_ratio(console: &mut Console, desired: f32, actual: f32) {
        // x / y = desired   cx / cy = actual  y/desired = cy/actual, y = cy * desired/actual
        console.scale_modifier = (1.0, 1.0);
        if desired > actual {
            console.scale_modifier.1 = actual / desired;
        } else {
            console.scale_modifier.0 = desired / actual;
        }
        console.set_dirty();
    }
}

#[derive(Clone)]
struct FontInfo {
    texture: Texture,
    glyph_map: HashMap<char, BoundingBox>,
    texture_scale: (i32, i32),
}

pub struct Console {
    is_dirty: RefCell<Dirty>,
    num_vert: RefCell<Num>,
    vao: VertexArray,
    vbo: ArrayBuffer,
    ebo: ElementArrayBuffer,
    font_info: FontInfo,
    glyphs: HashMap<(u32, u32), Glyph>,
    program: Program,
    dimensions: (u32, u32),
    screen_scaling:(f32, f32),
    height: u32,
    screen_offset: (f32, f32),
    default_background: data::f32_f32_f32_f32,
    scale_modifier: (f32, f32),
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

struct RelativeConsole {
    scale: (f32, f32),
    offset: (f32, f32),
}

pub struct ConsoleBuilder {
    size: (u32, u32),
    scale: (f32, f32),
    offset: (f32, f32),
    layer: u32,
    background: Color,
    font: String,
    relative: Option<RelativeConsole>,
    centered: bool,
    font_size: Option<(f32, f32)>,
    font_info: Option<FontInfo>,
}

impl ConsoleBuilder {
    pub fn with_dimensions(size: (u32, u32)) -> Self {
        ConsoleBuilder {
            size,
            scale: (1.0, 1.0),
            offset: (0.0, 0.0),
            layer: 1,
            background: *colors::BLACK,
            font: "droid-sans-mono.ttf".to_string(),
            relative: None,
            centered: false,
            font_size: None,
            font_info: None,
        }
    }

    pub fn with_font_size(size: (f32, f32)) -> Self {
        ConsoleBuilder {
            size: ((1.0 / size.0) as u32 , (1.0 / size.1) as u32),
            scale: (1.0, 1.0),
            offset: (0.0, 0.0),
            layer: 1,
            background: *colors::BLACK,
            font: "droid-sans-mono.ttf".to_string(),
            relative: None,
            centered: false,
            font_size: Some(size),
            font_info: None,
        }
    }

    pub fn with_dimensions_and_font_size(dimensions: (u32, u32), font_size: (f32, f32)) -> Self {
        ConsoleBuilder {
            size: dimensions,
            scale: (font_size.0 * dimensions.0 as f32, font_size.1 * dimensions.1 as f32),
            offset: (0.0, 0.0),
            layer: 1,
            background: *colors::BLACK,
            font: "droid-sans-mono.ttf".to_string(),
            relative: None,
            centered: false,
            font_size: Some(font_size),
            font_info: None,
        }
    }

    pub fn scale(&mut self, scale: (f32, f32)) -> &mut Self {
        self.size = match self.font_size {
            None => self.size,
            Some(size) => ((scale.0 / size.0) as u32, (scale.1 / size.1) as u32)
        };
        self.scale = scale;
        self
    }

    pub fn hscale(&mut self, scale: f32) -> &mut Self {
        self.scale((scale, self.scale.1));
        self
    }

    pub fn vscale(&mut self, scale: f32) -> &mut Self {
        self.scale((self.scale.0, scale));
        self
    }

    // relative to bottom left
    pub fn offset(&mut self, offset: (f32, f32)) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn layer(&mut self, layer: u32) -> &mut Self {
        self.layer = layer;
        self
    }

    pub fn background(&mut self, background: Color) -> &mut Self {
        self.background = background;
        self
    }

    pub fn font(&mut self, font: &str) -> &mut Self {
        self.font = font.to_string();
        self
    }

    pub fn centered(&mut self, centered: bool) -> &mut Self {
        self.centered = centered;
        self
    }

    pub fn top_align(&mut self) -> &mut Self {
        self.offset = (self.offset.0, (1.0 - self.scale.1) * 2.0);
        self
    }

    pub fn left_align(&mut self) -> &mut Self {
        self.offset = ((1.0 - self.scale.0) * 2.0, self.offset.1);
        self
    }

    pub fn right_align(&mut self) -> &mut Self {
        self.offset = ((1.0 + 3.0 * self.scale.0), self.offset.1);
        self
    }

    pub fn relative_to(&mut self, console: &Console) -> &mut Self {
        self.relative = Some(
            RelativeConsole {
                scale: console.screen_scaling,
                offset: console.screen_offset,
            }
        );
        self
    }

    pub fn font_from(&mut self, console: &Console) -> &mut Self {
        self.font_info = Some(console.font_info.clone());
        self
    }

    pub fn build(&self, res: &Resources, gl: &gl::Gl) -> Result<Console, failure::Error> {
        // Left bias the offset
        let offset = if self.centered {
            self.offset // (self.offset.1 + self.scale.0 / 2.0, self.offset.1 + self.scale.0 / 2.0)
        } else {
            (self.offset.0 - (1.0 - self.scale.0), self.offset.1 - (1.0 - self.scale.1))
        };
        match &self.relative {
            None => Console::new(res, gl, self.size, self.scale, offset, self.background, self.layer, self.font_info.clone()),
            Some(relative) => {
                let offset = (offset.0 + relative.offset.0, offset.1 + relative.offset.1);
                let scale = (self.scale.0 * relative.scale.0, self.scale.1 * relative.scale.1);
                Console::new(res, gl, self.size, scale, offset, self.background, self.layer, self.font_info.clone())
            }
        }
    }
}

impl Console {
    fn new(res: &Resources,
               gl: &gl::Gl,
               map_size: (u32, u32),
               screen_scaling: (f32, f32),
               screen_offset: (f32, f32),
               background: Color,
               height: u32,
               font: Option<FontInfo>) -> Result<Self, failure::Error> {
        let shader_program = render_gl::Program::from_res(
            &gl, &res, "shaders/glyph",
        )?;

        let font_info = match font {
            Some(font) => font,
            None => {
                let font_bytes = res.load_bytes_from_file("droid-sans-mono.ttf").unwrap();
                let (font_img, glyph_map) = load_bitmap(font_bytes);
                let texture_scale_u32 = font_img.dimensions();
                let texture = Texture::from_img(gl, font_img, gl::RGBA)?;
                let texture_scale = (texture_scale_u32.0 as i32, texture_scale_u32.1 as i32);
                FontInfo {
                    glyph_map,
                    texture_scale,
                    texture,
                }
            }
        };

        let vao = VertexArray::new(&gl);
        let vbo = ArrayBuffer::new(&gl);
        let ebo = ElementArrayBuffer::new(&gl);

        Ok(Console {
            is_dirty: RefCell::new(Dirty(true)),
            num_vert: RefCell::new(Num(0)),
            vao,
            vbo,
            ebo,
            font_info,
            height,
            glyphs: HashMap::new(),
            program: shader_program,
            dimensions: map_size,
            screen_scaling,
            screen_offset,
            default_background: background,
            scale_modifier: (1.0, 1.0)
        })
    }

    pub fn fill_background(&mut self, background: Color) {
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                self.put_char(' ', x as i32, y as i32, background, Some(background), 1);
            }
        }
    }

    pub fn clear(&mut self) {
        self.glyphs.clear();
        self.is_dirty.borrow_mut().set(true);
    }

    pub fn put_text(&mut self, text: &String, x: i32, y: i32,foreground: data::f32_f32_f32_f32, background: Option<data::f32_f32_f32_f32>, layer: u32) {
        for (index, c) in text.chars().enumerate() {
            self.put_char(c, x + index as i32, y, foreground, background, layer);
        }
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
        self.glyphs.insert((self.coordinates_to_index(x as u32, y as u32), layer), Glyph::new(c, background, foreground));
    }

    fn glyph_size(&self) -> (f32, f32) {
        (2.0 / self.dimensions.0 as f32 * self.screen_scaling.0 * self.scale_modifier.0, 2.0 / self.dimensions.1 as f32 * self.screen_scaling.1 * self.scale_modifier.1)
    }

    fn coordinates_to_index(&self, x: u32, y: u32) -> u32 {
        x + y * self.dimensions.0
    }

    fn index_to_coordinates(&self, index: u32) -> (u32, u32) {
        (index % self.dimensions.0, index / self.dimensions.0)
    }

    fn coordinates_to_fractional(&self, coordinates: (u32, u32)) -> (f32, f32) {
        (((coordinates.0 as f32 / self.dimensions.0 as f32) * 2.0 - 1.0)  * self.screen_scaling.0 * self.scale_modifier.0 + self.screen_offset.0,
         ((coordinates.1 as f32 / self.dimensions.1 as f32) * 2.0 - 1.0) * self.screen_scaling.1 * self.scale_modifier.1 + self.screen_offset.1 )
    }

    fn load_gl(&self, gl: &gl::Gl) -> i32 {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<gl::types::GLuint> = vec![];

        let mut num_glyphs = 0;
        for (index, glyph) in self.glyphs.iter() {
            let bounding_box = self.font_info.glyph_map.get(&glyph.character).unwrap();
            let scaled_bounding_box = self.glyph_size();
            let (index, layer) = *index;
            let layer = layer as f32 / 255.0 * -1.0 * self.height as f32;
            let coordinates = self.coordinates_to_fractional(self.index_to_coordinates(index));
            let index_offset = vertices.len() as u32;

            vertices.append(&mut vec![
                Vertex { position: (scaled_bounding_box.0 + coordinates.0, scaled_bounding_box.1 + coordinates.1, layer).into(),
                    texture: bounding_box.top_right(self.font_info.texture_scale).into(),
                    foreground: glyph.foreground,
                    background: glyph.background},
                Vertex { position: (scaled_bounding_box.0 + coordinates.0, coordinates.1, layer ).into(),
                    texture: bounding_box.bottom_right(self.font_info.texture_scale).into(),
                    foreground: glyph.foreground,
                    background: glyph.background },
                Vertex { position: (coordinates.0, coordinates.1, layer).into(),
                    texture: bounding_box.bottom_left(self.font_info.texture_scale).into(),
                    foreground: glyph.foreground,
                    background: glyph.background },
                Vertex { position: (coordinates.0, scaled_bounding_box.1 + coordinates.1, layer).into(),
                    texture: bounding_box.top_left(self.font_info.texture_scale).into(),
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

    fn set_dirty(&mut self) {
        self.is_dirty.borrow_mut().set(true);
    }

    pub fn render(&self, gl: &gl::Gl) {
        unsafe {
            gl.Enable(gl::DEPTH_TEST);
        }
        if self.is_dirty.borrow().0 {
            let num_glyphs = self.load_gl(&gl);
            self.is_dirty.borrow_mut().set(false);
            self.num_vert.borrow_mut().set(num_glyphs);
        }
        self.program.set_used();

        self.font_info.texture.bind();

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
