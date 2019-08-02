#![feature(duration_float)]
extern crate nalgebra;
extern crate nalgebra_glm;
extern crate image;
extern crate rand;
extern crate tobj;
extern crate num;
extern crate console_backend;

mod game;
mod game_handler;
mod map;
mod object;
mod debug;

use crate::debug::failure_to_string;

use crate::game_handler::GameHandler;
use crate::game::GameImpl;


pub fn main() {
    let mut game_handler = GameHandler::default();
    if let Err(e) = game_handler.run::<GameImpl>() {
        println!("{}", failure_to_string(e));
    }
}
