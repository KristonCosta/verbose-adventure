use crate::render_gl;
use crate::render_gl::{buffer, data};
use crate::resources::Resources;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray, ElementArrayBuffer};

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    position: data::f32_f32_f32,
    #[location = 1]
    color: data::f32_f32_f32,
}

pub struct Rectangle {
    program: render_gl::Program,
    _vbo: buffer::ElementArrayBuffer,
    vao: buffer::VertexArray,
}

impl Rectangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Self, failure::Error> {
        let shader_program = render_gl::Program::from_res(
            &gl, &res, "shaders/triangle"
        )?;

        let vertices: Vec<Vertex> = vec![
            Vertex { position: (0.5, 0.5, 0.0).into(),  color: (1.0, 0.0, 0.0).into()},
            Vertex { position: (0.5, -0.5, 0.0).into(),  color: (1.0, 0.0, 0.0).into()},
            Vertex { position: (-0.5, -0.5, 0.0).into(),  color: (0.0, 1.0, 0.0).into()},
            Vertex { position: (-0.5,  0.5, 0.0).into(),  color: (0.0, 0.0, 1.0).into()},
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

        Ok(Rectangle {
            program: shader_program,
            _vbo: ebo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
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