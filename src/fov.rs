use crate::map::Map;
use std::collections::HashSet;

pub fn calculate_fov(pos: (i32, i32), radius: u32, map: &Map) -> HashSet<(i32, i32)> {
    let mut set = HashSet::new();
    for i in 0..361 {
        calculate_fov_for_direction(pos, radius, ((i as f32).to_radians().cos(), (i as f32).to_radians().sin()), map, &mut set);
    }
    set
}

fn calculate_fov_for_direction(pos: (i32, i32), radius: u32, direction: (f32, f32), map: &Map, set :&mut HashSet<(i32, i32)>) {
    let mut ox = (pos.0 as f32) + 0.5;
    let mut oy = (pos.1 as f32) + 0.5;
    for i in 0..radius {
        set.insert((ox as i32, oy as i32));
        if map[ox as usize][oy as usize].block_sight {
            return
        }
        ox += direction.0;
        oy += direction.1;
    }
}