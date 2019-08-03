use crate::game_handler::{GameContext, InputEvent, InputEventData};

use glutin::{
    dpi::LogicalSize,
    window::Window,
    event::VirtualKeyCode,
};

use nalgebra_glm::Vec3;

use console_backend::{
    ColorBuffer,
    Camera,
    resources::Resources,
    Console,
    data,
    Color
};

use std::path::Path;
use std::collections::HashMap;
use std::time::Instant;
use failure::_core::time::Duration;
use crate::map::{Map, make_map, move_by};
use crate::object::{Object, Fighter, ai_take_turn, mut_two};
use std::cmp;

pub trait Game {
    fn new(context: &GameContext, size: LogicalSize) -> Self;
    fn render(&mut self, context: &GameContext);
    fn update(&mut self, pending_input: Option<InputEvent>, _dt: f32, _context: &GameContext);
    // fn process_input(&mut self, pending_input: InputEvent, context: &GameContext);
}

#[derive(PartialEq)]
enum PlayerAction {
    TookTurn,
    DidNotTakeTurn,
    Exit,
}


pub struct GameImpl {
    color_buffer: ColorBuffer,
    camera: Camera,
    console: Console,
    map: Map,
    objects: Vec<Object>,
    keyboard: HashMap<VirtualKeyCode, bool>,
    input_limiter: Instant,
    // canvas: CanvasRenderingContext2D,
 //   pending_action:
}

impl GameImpl {
    fn process_input(&mut self, pending_input: InputEvent, _context: &GameContext) -> PlayerAction {
        match pending_input {
            InputEvent::KeyPressed(
                InputEventData {
                data: key,
                ..
            }) => {
                match key {
                    VirtualKeyCode::W => {
                        self.player_move_or_attack(0, 1);
                        PlayerAction::TookTurn
                    },
                    VirtualKeyCode::S => {
                        self.player_move_or_attack(0, -1);
                        PlayerAction::TookTurn
                    },
                    VirtualKeyCode::A => {
                        self.player_move_or_attack(-1, 0);
                        PlayerAction::TookTurn
                    },
                    VirtualKeyCode::D => {
                        self.player_move_or_attack(1, 0);
                        PlayerAction::TookTurn
                    },
                    _ => PlayerAction::DidNotTakeTurn
                }
            }
            _ => PlayerAction::DidNotTakeTurn
        }
    }

    fn player_move_or_attack(&mut self, dx: i32, dy: i32) {
        let (x, y) = (self.objects[0].position.0 + dx,  self.objects[0].position.1 + dy);
        let target_id = self.objects.iter().position(|obj| obj.position == (x, y));
        match target_id {
            Some(target_id) => {
                let (player, monster) = mut_two(0, target_id, &mut self.objects);
                player.attack(monster);
            }
            None => {
                move_by(0, dx, dy, &self.map, &mut self.objects);
            }
        }
    }
}

impl Game for GameImpl  {
    fn new(context: &GameContext, size: LogicalSize) -> Self {
        let window: &Window = context.window.window();
        let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

        let physical_size = size.to_physical(window.hidpi_factor());
        context.window.resize(physical_size);

        let color_buffer = ColorBuffer::from_color(nalgebra::Vector3::new(0.3,0.3,0.5));
        color_buffer.set_used(&context.gl);

        let camera = Camera::new(size, Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 0.0, 0.0), &window);
        let map_size = (80, 50);
        let console = Console::new(&res, &context.gl,map_size, (size.width, (size.height * 0.75)).into(), data::f32_f32_f32_f32::new(255.0, 255.0, 255.0, 1.0)).unwrap();
        let input_map: HashMap<VirtualKeyCode, bool> = [].iter().cloned().collect();
        let mut objects = vec![];
        let white: Color = Color::from_int(200, 200,200,1.0);
        objects.push(Object::new((0, 0), '@', white, "player", true));

        let (map, player_pos) = make_map(map_size.0 as usize, map_size.1 as usize, 15, 5, 20, &mut objects);
        let player = &mut objects[0];
        // let font_context = CanvasFontContext::
        player.position = player_pos;
        player.fighter = Some(Fighter {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        });
        GameImpl {
            color_buffer,
            camera,
            console,
            objects,
            map,

            keyboard:input_map,
            input_limiter: Instant::now(),
        }
    }

    fn render(&mut self, context: &GameContext) {
        let gl = &context.gl;
        let color_dark_wall: Color = Color::from_int(0, 0, 100, 1.0);
        let color_dark_ground: Color = Color::from_int(50, 50, 150, 1.0);
        let clear: Color = Color::from_int(0, 0, 0, 0.0);
        self.color_buffer.clear(&gl);
        self.console.clear();
        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                if !(self.map[x][y].block_sight) {
                    self.console.put_char(
                        ' ',
                        x as i32, y as i32, clear, Some(color_dark_ground), 1);

                } else {
                    self.console.put_char(
                        ' ',
                        x as i32, y as i32, clear, Some(color_dark_wall), 1)
                }
            }
        }
        // self.console.render(&gl);
       // self.console.clear();
        for obj in self.objects.iter() {
            self.console.put_char(obj.glyph, obj.position.0, obj.position.1, obj.color, Some(clear), 2);
        }
        self.console.render(&gl);
    }

    fn update(&mut self, pending_input: Option<InputEvent>, _dt: f32, context: &GameContext) {
        if let Some(input) = pending_input {
            let action = self.process_input(input, context);
            if action == PlayerAction::TookTurn {
                for id in 1..self.objects.len() {
                    ai_take_turn(id, &self.map, &mut self.objects, true);
                }
            }
        }
    }
}