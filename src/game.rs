use crate::game_handler::{GameContext, InputEvent, InputEventData};

use glutin::{
    dpi::LogicalSize,
    window::Window,
    event::VirtualKeyCode,
};

use nalgebra_glm::Vec3;

use console_backend::{ColorBuffer, Camera, resources::Resources, Console, data, Color, colors, ConsoleBuilder, Transformer};

use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use failure::_core::time::Duration;
use crate::map::{Map, make_map, move_by};
use crate::object::{Object, Fighter, ai_take_turn, mut_two, DeathCallback, Item};
use crate::widgets::scrolling_message_console::ScrollingMessageConsole;
use crate::theme::theme;
use std::cmp;
use crate::fov::calculate_fov;
use crate::widgets::menu::Menu;

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


enum UseResult {
    UsedUp,
    Cancelled,
}

pub struct GameImpl {
    has_moved: bool,
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
    game_over: Console,
    active_menu: Option<Menu>,
    font_size: (f32, f32),
}

impl GameImpl {
    fn cast_heal(&mut self, _inventory_id: usize) -> UseResult {
        if let Some(fighter) = &self.objects[0].fighter {
            if fighter.hp == fighter.max_hp {
                self.message_log.add_colored_message("You are already at full health", *theme::RED_ALERT_TEXT);
                UseResult::Cancelled
            } else {
                self.message_log.add_colored_message("Your wounds start to feel better", *theme::GREEN_ALERT_TEXT);
                self.objects[0].heal(5);
                UseResult::UsedUp
            }
        } else {
            UseResult::Cancelled
        }
    }

    fn cast_lightning(&mut self, _inventory_id: usize) -> UseResult {
        self.message_log.add_colored_message("Your lightning fizzled.", *theme::RED_ALERT_TEXT);
        UseResult::UsedUp
    }

    fn inventory_menu(&mut self, header: String, context: &GameContext) {
        let options = if self.inventory.len() == 0 {
            vec!["Inventory is empty.".into()]
        } else {
          self.inventory.iter().map(|item| {item.name.clone()}).collect()
        };
        let mut max = header.len();
        options.iter().for_each(|option| {
            if option.len() > max {
                max = option.len()
            }});
        let menu = Menu::new(&context.gl,
                             header,
                             options,
                             (max + 1) as u32,
                             self.font_size,
                             &self.console);

        self.active_menu = Some(menu);
    }

    fn use_item(&mut self, inventory_id: usize) {
        if let Some(item) = &self.inventory[inventory_id].item {
            let on_use = match item {
                Item::Heal => GameImpl::cast_heal,
                Item::Lightning=> GameImpl::cast_lightning,
            };
            match on_use(self, inventory_id) {
                UseResult::UsedUp => {
                    self.inventory.remove(inventory_id);
                }
                UseResult::Cancelled => {
                    self.message_log.add_message("Cancelled");
                }
            }
        } else {
            self.message_log.add_message(&format!("The {} cannot be used.", self.inventory[inventory_id].name));
        }
    }

    fn process_input(&mut self, pending_input: InputEvent, context: &GameContext) -> PlayerAction {
        match pending_input {
            InputEvent::KeyPressed(
                InputEventData {
                data: key,
                ..
            }) => {
                self.has_moved = true;
                if let Some(menu) = &mut self.active_menu {
                    let index = menu.process_input(key);
                    if let Some(index) = index {
                        self.use_item(index);
                    }
                    self.active_menu = None;
                    PlayerAction::DidNotTakeTurn
                } else {
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
                        VirtualKeyCode::I => {
                            self.inventory_menu("Press the key next to an item to use it, or any other to cancel.".to_string(), &context);
                            PlayerAction::DidNotTakeTurn
                        }
                        _ => PlayerAction::DidNotTakeTurn
                    }
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
        let transformer = Transformer::AspectRatio(16.0 / 12.0, (size.width / size.height) as f32);
        transformer.apply(&mut self.console)
            .apply(&mut self.console_term)
            .apply(&mut self.game_over)
            .apply(&mut self.message_log.console);
        if let Some(menu) = &mut self.active_menu {
            transformer.apply(&mut menu.console);
        }
    }

    fn init_level_buffer(&mut self) {
        self.console.clear();
        let visible_tiles = calculate_fov(self.objects[0].position, 10, &self.map);
        for obj in self.objects.iter() {
            let layer = match obj.blocks {
                true => 3,
                false => 2,
            };
            if visible_tiles.contains(&(obj.position)) {
                self.console.put_char(obj.glyph, obj.position.0, obj.position.1, obj.color, Some(*colors::CLEAR), layer);
            }
        }
        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                if !(self.map[x][y].block_sight) {
                    let color = if visible_tiles.contains(&(x as i32, y as i32)) {
                        *theme::COLOR_LIGHT_FLOOR
                    } else {
                        *theme::COLOR_DARK_FLOOR
                    };
                    self.console.put_char(
                        ' ',
                        x as i32, y as i32, *colors::CLEAR, Some(color), 1);

                } else {
                    let color = if visible_tiles.contains(&(x as i32, y as i32)) {
                        *theme::COLOR_LIGHT_WALL
                    } else {
                        *theme::COLOR_DARK_WALL
                    };
                    self.console.put_char(
                        ' ',
                        x as i32, y as i32, *colors::CLEAR, Some(color), 1)
                }
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

        let color_buffer = ColorBuffer::from_color(nalgebra::Vector3::new(0.0,0.0,0.0));
        color_buffer.set_used(&context.gl);

        let camera = Camera::new(size, Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 0.0, 0.0), &window);
        let map_size = (100, 50);
        let font_size = (1.0 / 120.0, 1.0 / 40.0);
        let console = ConsoleBuilder::with_dimensions(map_size)
            .scale((1.0, 0.75))
            .top_align()
            .background(*theme::BACKGROUND)
            .layer(1)
            .build(&res, &context.gl)
            .unwrap();

        let console_term = ConsoleBuilder::with_dimensions((15, 8))
            .scale((0.2, 0.15))
            .font_from(&console)
            .top_align()
            .right_align()
            .background(*theme::BACKGROUND)
            .layer(2)
            .build(&res, &context.gl)
            .unwrap();

        let console_message_log = ConsoleBuilder::with_dimensions((120, 10))
                .scale((1.0, 0.25))
                .font_from(&console)
                .background(*theme::BACKGROUND)
                .layer(1)
                .build(&res, &context.gl)
                .unwrap();

        let game_over = ConsoleBuilder::with_dimensions((10, 1))
            .scale((0.5, 0.1))
            .font_from(&console)
            .background(*theme::BACKGROUND)
            .layer(10)
            .centered(true)
            .relative_to(&console)
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
            has_moved: true,
            color_buffer,
            camera,
            console,
            objects,
            map,
            console_term,
            message_log,
            game_over,
            inventory: vec![],
            keyboard:input_map,
            input_limiter: Instant::now(),
            active_menu: None,
            font_size,
        };
        game.set_window_ratios(size);
        game
    }



    fn render(&mut self, context: &GameContext) {
        let gl = &context.gl;
        self.color_buffer.clear(&gl);

        if self.has_moved {
            self.init_level_buffer();
            self.has_moved = false;
        }

        self.console.render(&gl);
        self.console_term.clear();
        if let Some(fighter) = self.objects[0].fighter {
            self.console_term.put_text(&format!("HP: {}/{}", fighter.hp, fighter.max_hp), 0, 7, *theme::PLAYER, Some(*colors::CLEAR), 2);
        }

        self.console_term.render(&gl);
        self.message_log.render(&gl);
        if let Some(menu) = &mut self.active_menu {
            menu.render(&gl);
        }
    }

    fn update(&mut self, pending_input: Option<InputEvent>, _dt: f32, context: &GameContext) {
        if let Some(input) = pending_input {
            let action = self.process_input(input, context);
            if action == PlayerAction::TookTurn {
                for id in 1..self.objects.len() {
                    if self.objects[id].ai.is_some() {
                        let has_moved = ai_take_turn(id, &self.map, &mut self.objects, true, &mut self.message_log);
                        if has_moved {
                            self.has_moved = true;
                        }
                    }
                }
            }
        }
    }

    fn resize(&mut self, size: LogicalSize) {
        self.set_window_ratios(size);
    }
}