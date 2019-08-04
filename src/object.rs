use console_backend::Color;
use crate::map::{Map, move_by};
use std::cmp;
use crate::scrolling_message_console::ScrollingMessageConsole;

pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert_ne!(first_index, second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

#[derive(Debug)]
pub enum Item {
    Heal,
}

#[derive(Debug)]
pub struct Object {
    pub position: (i32, i32),
    pub glyph: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
    pub item: Option<Item>,
}

impl Object {
    pub fn new(position: (i32, i32), glyph: char, color: Color, name: &str, blocks: bool) -> Self {
        Object {
            position,
            glyph,
            color,
            blocks,
            name: name.into(),
            alive: true,
            fighter: None,
            ai: None,
            item: None,
        }
    }

    pub fn distance_to(&self, other: &Object) -> f32 {
        let (dx, dy) = (other.position.0 - self.position.0, other.position.1 - self.position.1);
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32) {
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }
        if let Some(fighter) = self.fighter {
            if fighter.hp <= 0 {
                self.alive = false;
                fighter.on_death.callback(self);
            }
        }
    }

    pub fn attack(&self, target: &mut Object, messages: &mut ScrollingMessageConsole) {
        let damage = self.fighter.map_or(0, |f| f.power) - target.fighter.map_or(0, |f| f.defense);
        if damage > 0 {
            messages.add_message(format!(
                "{} attacks {} for {} hit points.",
                self.name, target.name, damage).as_str());
            target.take_damage(damage);
        } else {
            messages.add_message(format!(
                "{} attacks {} but it has no effect!",
                self.name, target.name
            ).as_str());
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub on_death: DeathCallback,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ai;

fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
    let (dx, dy) = (target_x - objects[id].position.0, target_y - objects[id].position.1);
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    let (dx, dy) = ((dx as f32 / distance).round() as i32, (dy as f32 / distance).round() as i32);
    move_by(id, dx, dy, map, objects);
}

pub fn ai_take_turn(monster_id: usize, map: &Map, mut objects: &mut [Object], fov_map: bool, messages: &mut ScrollingMessageConsole) {
    let (x, y) = objects[monster_id].position;
    if objects[monster_id].distance_to(&objects[0]) >= 2.0 {
        let (player_x, player_y) = objects[0].position;
        move_towards(monster_id, player_x, player_y, map, objects);
    } else if objects[0].fighter.map_or(false, |f| f.hp > 0) {
        let (monster, player) = mut_two(monster_id, 0, &mut objects);
        monster.attack(player, messages);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster
}
impl DeathCallback {
    fn callback(self, object: &mut Object) {
        use DeathCallback::*;
        let callback: fn(&mut Object) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(object);
    }
}

fn player_death(player: &mut Object) {
    println!("You died!");

    player.glyph = '%';
    player.color = Color::from_int(120, 30, 30, 1.0);
}

fn monster_death(monster: &mut Object) {
    println!("{} is dead!", monster.name);

    monster.glyph = '%';
    monster.color = Color::from_int(120, 30, 30, 1.0);
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);

}