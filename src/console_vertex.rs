use crate::render_gl::data;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    #[location = 0]
    pub position: data::f32_f32_f32,
    #[location = 1]
    pub texture: data::f32_f32,
    #[location = 2]
    pub background: data::f32_f32_f32_f32,
    #[location = 3]
    pub foreground: data::f32_f32_f32_f32,
}