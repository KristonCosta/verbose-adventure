use crate::render_gl;
use crate::render_gl::{buffer, data};
use crate::resources::Resources;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray, ElementArrayBuffer};
use crate::render_gl::texture::Texture;
use nalgebra_glm::{TVec, TVec1, RealField};
use crate::render_gl::math::radians;
use nalgebra::Vector3;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    position: data::f32_f32_f32,
    #[location = 1]
    texture: data::f32_f32,
}

pub struct Cube {
    pub program: render_gl::Program,
    texture: Texture,
    face: Texture,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Cube {
    pub fn new(res: &Resources, gl: &gl::Gl, offset: nalgebra::Vector3<f32>) -> Result<Self, failure::Error> {
        let shader_program = render_gl::Program::from_res(
            &gl, &res, "shaders/coordinate"
        )?;

        let texture = Texture::from_res(gl, res, "texture/container.jpg", gl::RGB)?;
        let face = Texture::from_res(gl, res, "texture/awesomeface.png", gl::RGBA)?;

        let x = unsafe { offset.get_unchecked(0) };
        let y = unsafe { offset.get_unchecked(1) };
        let z = unsafe { offset.get_unchecked(2) };

        let vertices: Vec<Vertex> = vec![
            Vertex {position: (-0.5 + *x, -0.5 + *y, -0.5 + *z).into(), texture: (0.0, 0.0).into()},
            Vertex {position: ( 0.5 + *x, -0.5 + *y, -0.5 + *z).into(), texture: (1.0, 0.0).into()},
            Vertex {position: ( 0.5 + *x,  0.5 + *y, -0.5 + *z).into(), texture: (1.0, 1.0).into()},
            Vertex {position: ( 0.5 + *x,  0.5 + *y, -0.5 + *z).into(), texture: (1.0, 1.0).into()},
            Vertex {position: (-0.5 + *x,  0.5 + *y, -0.5 + *z).into(), texture: (0.0, 1.0).into()},
            Vertex {position: (-0.5 + *x, -0.5 + *y, -0.5 + *z).into(), texture: (0.0, 0.0).into()},
            Vertex {position: (-0.5 + *x, -0.5 + *y,  0.5 + *z).into(), texture: (0.0, 0.0).into()},
            Vertex {position: ( 0.5 + *x, -0.5 + *y,  0.5 + *z).into(), texture: (1.0, 0.0).into()},
            Vertex {position: ( 0.5 + *x,  0.5 + *y,  0.5 + *z).into(), texture: (1.0, 1.0).into()},
            Vertex {position: ( 0.5 + *x,  0.5 + *y,  0.5 + *z).into(), texture: (1.0, 1.0).into()},
            Vertex {position: (-0.5 + *x,  0.5 + *y,  0.5 + *z).into(), texture: (0.0, 1.0).into()},
            Vertex {position: (-0.5 + *x, -0.5 + *y,  0.5 + *z).into(), texture: (0.0, 0.0).into()},
            Vertex {position: (-0.5 + *x,  0.5 + *y,  0.5 + *z).into(), texture: (1.0, 0.0).into()},
            Vertex {position: (-0.5 + *x,  0.5 + *y, -0.5 + *z).into(), texture: (1.0, 1.0).into()},
            Vertex {position: (-0.5 + *x, -0.5 + *y, -0.5 + *z).into(), texture: (0.0, 1.0).into()},
            Vertex {position: (-0.5 + *x, -0.5 + *y, -0.5 + *z).into(), texture: (0.0, 1.0).into()},
            Vertex {position: (-0.5 + *x, -0.5 + *y,  0.5 + *z).into(), texture: (0.0, 0.0).into()},
            Vertex {position: (-0.5 + *x,  0.5 + *y,  0.5 + *z).into(), texture: (1.0, 0.0).into()},
            Vertex {position: ( 0.5 + *x,  0.5 + *y,  0.5 + *z).into(), texture: (1.0, 0.0).into()},
            Vertex {position: ( 0.5 + *x,  0.5 + *y, -0.5 + *z).into(), texture: (1.0, 1.0).into()},
            Vertex {position: ( 0.5 + *x, -0.5 + *y, -0.5 + *z).into(), texture: (0.0, 1.0).into()},
            Vertex {position: ( 0.5 + *x, -0.5 + *y, -0.5 + *z).into(), texture: (0.0, 1.0).into()},
            Vertex {position: ( 0.5 + *x, -0.5 + *y,  0.5 + *z).into(), texture: (0.0, 0.0).into()},
            Vertex {position: ( 0.5 + *x,  0.5 + *y,  0.5 + *z).into(), texture: (1.0, 0.0).into()},
            Vertex {position: (-0.5 + *x, -0.5 + *y, -0.5 + *z).into(), texture: (0.0, 1.0).into()},
            Vertex {position: ( 0.5 + *x, -0.5 + *y, -0.5 + *z).into(), texture: (1.0, 1.0).into()},
            Vertex {position: ( 0.5 + *x, -0.5 + *y,  0.5 + *z).into(), texture: (1.0, 0.0).into()},
            Vertex {position: ( 0.5 + *x, -0.5 + *y,  0.5 + *z).into(), texture: (1.0, 0.0).into()},
            Vertex {position: (-0.5 + *x, -0.5 + *y,  0.5 + *z).into(), texture: (0.0, 0.0).into()},
            Vertex {position: (-0.5 + *x, -0.5 + *y, -0.5 + *z).into(), texture: (0.0, 1.0).into()},
            Vertex {position: (-0.5 + *x,  0.5 + *y, -0.5 + *z).into(), texture: (0.0, 1.0).into()},
            Vertex {position: ( 0.5 + *x,  0.5 + *y, -0.5 + *z).into(), texture: (1.0, 1.0).into()},
            Vertex {position: ( 0.5 + *x,  0.5 + *y,  0.5 + *z).into(), texture: (1.0, 0.0).into()},
            Vertex {position: ( 0.5 + *x,  0.5 + *y,  0.5 + *z).into(), texture: (1.0, 0.0).into()},
            Vertex {position: (-0.5 + *x,  0.5 + *y,  0.5 + *z).into(), texture: (0.0, 0.0).into()},
            Vertex {position: (-0.5 + *x,  0.5 + *y, -0.5 + *z).into(), texture: (0.0, 1.0).into()},
        ];
        let vao = VertexArray::new(&gl);
        let vbo = ArrayBuffer::new(&gl);


        vao.bind();

        vbo.bind();
        vbo.static_draw_data(&vertices);

        Vertex::vertex_attrib_pointers(&gl);

        vbo.unbind();
        vao.unbind();

        Ok(Cube {
            texture,
            face,
            program: shader_program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl, angle: f32, position: &Vector3<f32>) {
        self.program.set_used();
        self.program.set_int("texture1", 0);
        self.program.set_int("texture2", 1);
        let model = nalgebra_glm::identity();
        let model = nalgebra_glm::translate(&model, position);
        let model = nalgebra_glm::rotate(&model, angle, &nalgebra_glm::vec3(0.5, 1.0, 0.0));
        self.program.set_mat_4f("model", model);
        unsafe {
            gl.ActiveTexture(gl::TEXTURE0);
            self.texture.bind();
            gl.ActiveTexture(gl::TEXTURE1);
            self.face.bind();
        }
        self.vao.bind();
        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                36,
            );
        }
    }
}