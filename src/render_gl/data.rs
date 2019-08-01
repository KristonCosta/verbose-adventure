use gl;
use gl::Gl;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
#[allow(non_camel_case_types)]
pub struct f32_f32_f32_f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub a: f32,
}

impl f32_f32_f32_f32 {
    pub fn new(x: f32, y: f32, z: f32, a: f32) -> f32_f32_f32_f32 {
        f32_f32_f32_f32 {
            x, y, z, a
        }
    }
}

impl From<(f32, f32, f32, f32)> for f32_f32_f32_f32 {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        f32_f32_f32_f32::new(other.0, other.1, other.2, other.3)
    }
}

impl f32_f32_f32_f32 {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
#[allow(non_camel_case_types)]
pub struct f32_f32_f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl f32_f32_f32 {
    pub fn new(x: f32, y: f32, z: f32) -> f32_f32_f32 {
        f32_f32_f32 {
            x, y, z
        }
    }
}

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

impl f32_f32_f32 {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
#[allow(non_camel_case_types)]
pub struct f32_f32 {
    pub x: f32,
    pub y: f32,
}

impl f32_f32 {
    pub fn new(x: f32, y: f32) -> f32_f32 {
        f32_f32 {
            x, y,
        }
    }
}

impl From<(f32, f32)> for f32_f32 {
    fn from(other: (f32, f32)) -> Self {
        f32_f32::new(other.0, other.1)
    }
}

impl f32_f32 {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}