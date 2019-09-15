#![feature(duration_float)]
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;
#[macro_use] extern crate lazy_static;
mod console;
mod color;
mod console_vertex;
mod glyph;
pub mod resources;
mod render_gl;
mod renderable;

pub use render_gl::data;
pub use console::{
    Console,
    ConsoleBuilder,
    Transformer,
};

pub use color::{Color, colors};
pub use glyph::Glyph;
pub use render_gl::{
    camera::Camera,
    buffer::{VertexArray, ArrayBuffer, ElementArrayBuffer},
    color_buffer::ColorBuffer,
    viewport::Viewport,
};