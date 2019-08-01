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

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    position: data::f32_f32_f32,
    #[location = 1]
    texture: data::f32_f32,
}

pub struct Glyph {
    pub program: render_gl::Program,
    texture: Texture,
    _ebo: buffer::ElementArrayBuffer,
    vao: buffer::VertexArray,
}

impl Glyph {
    pub fn new(res: &Resources, gl: &gl::Gl, offset: nalgebra::Vector3<f32>, letter: char) -> Result<Self, failure::Error> {
        let shader_program = render_gl::Program::from_res(
            &gl, &res, "shaders/glyph"
        )?;
        let font_bytes = res.load_bytes_from_file("droid-sans-mono.ttf").unwrap();
        let (font_img, font_map) = load_bitmap(font_bytes);
        font_img.save("123.png");
        let texture_scale_u32 = font_img.dimensions();
       // let texture = Texture::from_res(&gl, &res, "texture/123.png", gl::RGBA)?;
        let texture = Texture::from_img(gl, font_img, gl::RGBA)?;
        let bounding_box = font_map.get(&letter).unwrap();
        println!("{:?}", bounding_box);

        let texture_scale = (texture_scale_u32.0 as i32, texture_scale_u32.1 as i32);
        let x = unsafe { offset.get_unchecked(0) };
        let y = unsafe { offset.get_unchecked(1) };
        let z = unsafe { offset.get_unchecked(2) };

        let vertices: Vec<Vertex> = vec![
            Vertex { position: (0.5, 0.5, 0.0).into(), texture: bounding_box.top_right(texture_scale).into()},
            Vertex { position: (0.5, -0.5, 0.0).into(), texture: bounding_box.bottom_right(texture_scale).into()},
            Vertex { position: (-0.5, -0.5, 0.0).into(), texture: bounding_box.bottom_left(texture_scale).into()},
            Vertex { position: (-0.5,  0.5, 0.0).into(), texture: bounding_box.top_left(texture_scale).into()},
        ];
        let indicies: Vec<gl::types::GLuint> = vec![
            0, 1, 3, 1, 2, 3,
        ];
        let vao = VertexArray::new(&gl);
        let vbo = ArrayBuffer::new(&gl);
        let ebo = ElementArrayBuffer::new(&gl);

        vao.bind();

        vbo.bind();
        vbo.static_draw_data(&vertices);

        ebo.bind();
        ebo.static_draw_data(&indicies);

        Vertex::vertex_attrib_pointers(&gl);

        vbo.unbind();
        vao.unbind();
        ebo.unbind();

        Ok(Glyph {
            texture,
            program: shader_program,
            _ebo: ebo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        unsafe {
            self.texture.bind();
        }
        self.vao.bind();
        unsafe {
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }
    }
}
