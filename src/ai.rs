use crate::element::Element;
use crate::fov::FovMap;
use crate::map::{
    Map,
    move_by,
};
use crate::utils::mut_two;

fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, elements: &mut [Element]) {
    let dx = target_x - elements[id].position.x;
    let dy = target_y - elements[id].position.y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize it to length 1 (preserving direction), then round it and
    // convert to integer so the movement is restricted to the map grid
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    move_by(id, map, elements, dx, dy);
}


pub fn ai_take_turn(monster_id: usize, map: &Map, elements: &mut [Element], player_id: usize, fov_map: &FovMap) {
    // a basic monster takes its turn. If you can see it, it can see you
    let (monster_x, monster_y) = elements[monster_id].pos();
    if fov_map.is_in_sight(monster_x, monster_y) {
        if elements[monster_id].distance_to(&elements[player_id]) >= 2.0 {
            // move towards player if far away
            let (player_x, player_y) = elements[player_id].pos();
            move_towards(monster_id, player_x, player_y, map, elements);
        } else if elements[player_id].fighter.map_or(false, |f| f.hp > 0) {
            // close enough, attack! (if the player is still alive.)
            let (monster, player) = mut_two(monster_id, player_id, elements);
            monster.attack(player);
        }
    }
}
