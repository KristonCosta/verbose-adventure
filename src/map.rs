use console_backend::Color;
use rand::Rng;
use std::cmp;
use crate::object::{Object, Fighter, Ai, DeathCallback, Item};
use crate::theme::theme;

const MAX_ROOM_ITEMS: u32 = 2;
const MAX_ROOM_MONSTERS: u32 = 3;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 < other.y2)
            && (self.y2 >= other.y1)
    }
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..=(cmp::max(x1, x2)) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..=(cmp::max(y1, y2)) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub type Map = Vec<Vec<Tile>>;

#[allow(clippy::ptr_arg)]
pub fn is_not_blocked(map: &Map, x: i32, y: i32, objects: &[Object]) -> bool {
    x >= 0
        && x < map.len() as i32
        && y >= 0
        && y < map[0].len() as i32
        && !map[x as usize][y as usize].blocked
        && !objects.iter().any(|obj| {
            obj.position == (x, y) && obj.blocks
    })
}


fn place_objects(map: &Map, room: Rect, objects: &mut Vec<Object>) {
    let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);
    for _ in 0..num_monsters {
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        let monster = if rand::random::<f32>() < 0.8 {
            let mut obj = Object::new((x, y), 'o', *theme::ORC, "orc", true);
            obj.fighter = Some(Fighter {
                max_hp: 10,
                hp: 10,
                defense: 0,
                power: 3,
                on_death: DeathCallback::Monster,
            });
            obj.ai = Some(Ai);
            obj
        } else {
            let mut obj = Object::new((x, y), 'T', *theme::TROLL, "troll", true);
            obj.fighter = Some(Fighter {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4,
                on_death: DeathCallback::Monster,
            });
            obj.ai = Some(Ai);
            obj
        };
        objects.push(monster);
    }

    let num_items = rand::thread_rng().gen_range(0, MAX_ROOM_ITEMS + 1);
    for _ in 0..num_items {
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if is_not_blocked(map, x, y, objects) {
            let die = rand::random::<f32>();
            let item = if die < 0.7 {
                let mut object = Object::new((x, y), '!', *theme::HEALING_ITEM, "healing potion",  false);
                object.item = Some(Item::Heal);
                object
            } else {
                let mut object = Object::new((x, y), '#', *theme::SCROLL_ITEM, "scroll of lightning", false);
                object.item = Some(Item::Lightning);
                object
            };
            objects.push(item);
        }
    }
}

pub fn make_map(
    width: usize,
    height: usize,
    room_max_size: i32,
    room_min_size: i32,
    max_rooms: i32,
    objects: &mut Vec<Object>,
) -> (Map, (i32, i32)) {
    let mut map = vec![vec![Tile::wall(); height]; width];
    let mut rooms = vec![];
    let mut starting_position = (0, 0);
    for _ in 0..max_rooms {
        let w = rand::thread_rng().gen_range(room_min_size, room_max_size + 1);
        let h = rand::thread_rng().gen_range(room_min_size, room_max_size + 1);
        let x = rand::thread_rng().gen_range(0, width - w as usize);
        let y = rand::thread_rng().gen_range(0, height - h as usize);
        let new_room = Rect::new(x as i32, y as i32, w, h);
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));
        if !failed {
            create_room(new_room, &mut map);
            place_objects(&map, new_room, objects);
            let (new_x, new_y) = new_room.center();
            if rooms.is_empty() {
                starting_position = (new_x, new_y);
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }
            rooms.push(new_room);
        }
    }
    (map, starting_position)
}


#[allow(clippy::ptr_arg)]
pub fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let (x, y) = objects[id].position;
    if is_not_blocked(map, x + dx, y + dy, objects) {
        objects[id].position = (x + dx, y + dy);
    }
}
