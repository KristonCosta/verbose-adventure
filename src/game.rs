use crate::game_handler::{GameContext, InputEvent, InputEventData};

use glutin::{
    dpi::LogicalSize,
    window::Window,
    event::VirtualKeyCode,
};

use nalgebra_glm::Vec3;

use console_backend::{ColorBuffer, Camera, resources::Resources, Console, data, Color, colors, ConsoleBuilder, Transformer};

use std::path::Path;
use std::collections::HashMap;
use std::time::Instant;
use failure::_core::time::Duration;
use crate::map::{Map, make_map, move_by};
use crate::object::{Object, Fighter, ai_take_turn, mut_two, DeathCallback};
use crate::scrolling_message_console::ScrollingMessageConsole;
use crate::theme::theme;
use std::cmp;

pub trait Game {
    fn new(context: &GameContext, size: LogicalSize) -> Self;
    fn render(&mut self, context: &GameContext);
    fn update(&mut self, pending_input: Option<InputEvent>, _dt: f32, _context: &GameContext);
    fn resize(&mut self, size: LogicalSize);
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
    inventory: Vec<Object>,
    console_term: Console,
    map: Map,
    objects: Vec<Object>,
    keyboard: HashMap<VirtualKeyCode, bool>,
    input_limiter: Instant,
    message_log: ScrollingMessageConsole,
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
                    VirtualKeyCode::G => {
                        let item_id = self.objects
                            .iter()
                            .position(|object|
                                object.position == self.objects[0].position && object.item.is_some());
                        if let Some(item_id) = item_id {
                            self.pick_item_up(item_id);
                        }
                        PlayerAction::DidNotTakeTurn
                    }
                    _ => PlayerAction::DidNotTakeTurn
                }

            }
            _ => PlayerAction::DidNotTakeTurn
        }
    }

    fn player_move_or_attack(&mut self, dx: i32, dy: i32) {
        if self.objects[0].alive {
            let (x, y) = (self.objects[0].position.0 + dx, self.objects[0].position.1 + dy);
            let target_id = self.objects.iter().position(|obj| obj.position == (x, y) && obj.fighter.is_some());
            match target_id {
                Some(target_id) => {
                    let (player, monster) = mut_two(0, target_id, &mut self.objects);
                    player.attack(monster, &mut self.message_log);
                }
                None => {
                    move_by(0, dx, dy, &self.map, &mut self.objects);
                }
            }
        }
    }

    fn pick_item_up(&mut self, item_id: usize) {
        if self.inventory.len() >= 2 {
            self.message_log.add_colored_message(
                format!("Your inventory is full, cannot pick up {}.", self.objects[item_id].name).as_str(),
                *theme::RED_ALERT_TEXT,
            )
        } else {
            let item = self.objects.swap_remove(item_id);
            self.message_log.add_colored_message(
                format!("You picked up {}!", item.name).as_str(),
                *theme::GREEN_ALERT_TEXT,
            );
            self.inventory.push(item);
        }
    }

    fn set_window_ratios(&mut self, size: LogicalSize) {
        Transformer::AspectRatio(16.0 / 10.0, (size.width / size.height) as f32)
            .apply(&mut self.console)
            .apply(&mut self.console_term)
            .apply(&mut self.message_log.console);
    }
}

impl Game for GameImpl  {
    fn new(context: &GameContext, size: LogicalSize) -> Self {
        let window: &Window = context.window.window();
        let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

        let physical_size = size.to_physical(window.hidpi_factor());
        context.window.resize(physical_size);

        let color_buffer = ColorBuffer::from_color(nalgebra::Vector3::new(0.0,0.0,0.0));
        color_buffer.set_used(&context.gl);

        let camera = Camera::new(size, Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 0.0, 0.0), &window);
        let map_size = (100, 40);
        let console = ConsoleBuilder::new(map_size)
            .scale((1.0, 0.75))
            .top_align()
            .background(*theme::BACKGROUND)
            .layer(1)
            .build(&res, &context.gl)
            .unwrap();

        let console_term = ConsoleBuilder::new((15, 8))
            .scale((0.2, 0.15))
            .top_align()
            .right_align()
            .background(*theme::BACKGROUND)
            .layer(2)
            .build(&res, &context.gl)
            .unwrap();

        let console_message_log = ConsoleBuilder::new((map_size.0 + 20, 10))
                .scale((1.0, 0.25))
                .background(*theme::BACKGROUND)
                .layer(1)
                .build(&res, &context.gl)
                .unwrap();

        let mut message_log = ScrollingMessageConsole::new(console_message_log, 10);
        message_log.add_colored_message("Oh man this is spooky.", *theme::RED_ALERT_TEXT);
        let input_map: HashMap<VirtualKeyCode, bool> = [].iter().cloned().collect();
        let mut objects = vec![];
        objects.push(Object::new((0, 0), '@', *theme::PLAYER, "player", true));

        let (map, player_pos) = make_map(map_size.0 as usize, map_size.1 as usize, 15, 5, 20, &mut objects);
        let player = &mut objects[0];

        player.position = player_pos;
        player.fighter = Some(Fighter {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
            on_death: DeathCallback::Player,
        });
        let mut game = GameImpl {
            color_buffer,
            camera,
            console,
            objects,
            map,
            console_term,
            message_log,
            inventory: vec![],
            keyboard:input_map,
            input_limiter: Instant::now(),
        };
        game.set_window_ratios(size);
        game
    }

    fn render(&mut self, context: &GameContext) {
        let gl = &context.gl;
        unsafe {
          //  gl.PolygonMode( gl::FRONT_AND_BACK, gl::LINE );

        }
        self.color_buffer.clear(&gl);
        self.console.clear();
        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                if !(self.map[x][y].block_sight) {
                    self.console.put_char(
                        ' ',
                        x as i32, y as i32, *colors::CLEAR, Some(*theme::COLOR_DARK_FLOOR), 1);

                } else {
                    self.console.put_char(
                        ' ',
                        x as i32, y as i32, *colors::CLEAR, Some(*theme::COLOR_DARK_WALL), 1)
                }
            }
        }
        for obj in self.objects.iter() {
            let layer = match obj.blocks {
                true => 3,
                false => 2,
            };
            self.console.put_char(obj.glyph, obj.position.0, obj.position.1, obj.color, Some(*colors::CLEAR), layer);
        }
        self.console.render(&gl);
        self.console_term.clear();

        if let Some(fighter) = self.objects[0].fighter {
            let mut count = 0;
            for char in format!("HP: {}/{}", fighter.hp, fighter.max_hp).chars() {
                self.console_term.put_char(char, count, 7, *theme::PLAYER, Some(*colors::CLEAR), 2);
                count += 1;
            }
        }
        self.console_term.render(&gl);
        self.message_log.render(&gl);
    }

    fn update(&mut self, pending_input: Option<InputEvent>, _dt: f32, context: &GameContext) {
        if let Some(input) = pending_input {
            let action = self.process_input(input, context);
            if action == PlayerAction::TookTurn {
                for id in 1..self.objects.len() {
                    if self.objects[id].ai.is_some() {
                        ai_take_turn(id, &self.map, &mut self.objects, true, &mut self.message_log);
                    }
                }
            }
        }
    }

    fn resize(&mut self, size: LogicalSize) {
        self.set_window_ratios(size);
    }
}