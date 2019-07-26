use crate::render_gl;
use crate::render_gl::{buffer, data};
use crate::resources::Resources;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray};

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    position: data::f32_f32_f32,
    #[location = 1]
    color: data::f32_f32_f32,
}

pub struct Triangle {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Self, failure::Error> {
        let shader_program = render_gl::Program::from_res(
            &gl, &res, "shaders/triangle"
        )?;

        let vertices: Vec<Vertex> = vec![
            Vertex { position: (-0.5, -0.5, 0.0).into(),  color: (1.0, 0.0, 0.0).into()},
            Vertex { position: (0.5, -0.5, 0.0).into(),  color: (0.0, 1.0, 0.0).into()},
            Vertex { position: (0.0,  0.5, 0.0).into(),  color: (0.0, 0.0, 1.0).into()},
        ];

        let vbo = ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();
        let vao = VertexArray::new(&gl);
        vao.bind();
        vbo.bind();

        Vertex::vertex_attrib_pointers(&gl);

        vbo.unbind();
        vao.unbind();
        Ok(Triangle {
            program: shader_program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }
    }
}