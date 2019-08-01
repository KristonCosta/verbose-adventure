#![feature(duration_float)]
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;
extern crate nalgebra;
extern crate nalgebra_glm;
extern crate image;
extern crate rand;
extern crate tobj;
extern crate num;

use num::Float;
pub mod render_gl;
pub mod resources;
mod cube;
mod triangle;
mod rectangle;
mod debug;
mod game;
mod glyph;
mod game_handler;
mod console;
mod console_vertex;
mod map;
mod object;
mod color;

use gl::Gl;
use render_gl::{data, color_buffer};

use resources::Resources;
use std::path::Path;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray};
use crate::debug::failure_to_string;
use crate::render_gl::viewport::Viewport;
use crate::render_gl::color_buffer::ColorBuffer;
use nalgebra::Matrix4;
use std::time::Instant;
use nalgebra_glm::{RealField, Vec3, U3};
use crate::render_gl::math::radians;
use crate::render_gl::camera::Camera;
use failure::_core::time::Duration;
use std::collections::HashMap;
use crate::game_handler::GameHandler;
use crate::game::GameImpl;


pub fn main() {
    let mut game_handler = GameHandler::default();
    if let Err(e) = game_handler.run::<GameImpl>() {
        println!("{}", failure_to_string(e));
    }
}
