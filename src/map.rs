extern crate rand;

use crate::{
    constants::{
        MAP_WIDTH,
        MAP_HEIGHT,
        ROOM_MAX_SIZE,
        ROOM_MIN_SIZE,
        MAX_ROOMS,
        MAX_ROOM_MONSTERS,
        MAX_ROOM_ITEMS,
    },
    element::*,
    fov::FovMap,
};
use rand::Rng;
use std::cmp;

pub type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub block_movement: bool,
    pub block_sight: bool,
    pub explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { block_movement: false, block_sight: false, explored: false }
    }
    pub fn wall() -> Self {
        Tile { block_movement: true, block_sight: true, explored: false }
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
        (self.x1 <= other.x2) && (self.x2 >= other.x1) &&
            (self.y1 <=  other.y2) && (self.y2 >= other.y1)
    }
}

pub fn generate_map(elements: &mut Vec<Element>) -> (Map, (i32, i32)) {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let mut starting_position = (0, 0);
    let mut rooms: Vec<Rect> = vec![];
    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        // run through the other rooms and see if they intersect with this one
        let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));
        if !failed {
            create_room(new_room, &mut map);
            place_elements(&map, new_room, elements);
            let (new_room_x, new_room_y) = new_room.center();
            if rooms.is_empty() {
                starting_position = (new_room_x, new_room_y);
            } else {
                let (prev_room_x, prev_room_y) = rooms[rooms.len() - 1].center();

                if rand::random() {
                    create_h_tunnel(prev_room_x, new_room_x, prev_room_y, &mut map);
                    create_v_tunnel(prev_room_y, new_room_y, new_room_x, &mut map);
                } else {
                    create_v_tunnel(prev_room_y, new_room_y, prev_room_x, &mut map);
                    create_h_tunnel(prev_room_x, new_room_x, new_room_y, &mut map);
                }
            }
            rooms.push(new_room);
        }
    }

    (map, starting_position)
}

pub fn explore(map: &mut Map, fov_map: &FovMap) {
    // TODO Do on the FovMap instead
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let is_visible = fov_map.is_in_sight(x, y);

            let explored = &mut map[x as usize][y as usize].explored;
            if is_visible {
                *explored = true;
            }
        }
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
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn place_elements(map: &Map, room: Rect, elements: &mut Vec<Element>) {
    let num_monsters =  rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

    for _ in 0..num_monsters {
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !is_blocked(x, y, map, elements) {
            let monster = if rand::random::<f32>() < 0.8 {
                make_orc(x, y)
            } else {
                make_troll(x, y)
            };
            elements.push(monster);
        }
    }

    // choose random number of items
    let num_items = rand::thread_rng().gen_range(0, MAX_ROOM_ITEMS + 1);

    for _ in 0..num_items {
        // choose random spot for this item
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        // only place it if the tile is not blocked
        if !is_blocked(x, y, map, elements) {
            // create a healing potion
            let element = make_potion(x, y);
            elements.push(element);
        }
    }


}

fn is_blocked(x : i32, y: i32, map: &Map, elements: &[Element]) -> bool {
    if map[x as usize][y as usize].block_movement {
        return true;
    }

    elements.iter().any(|element| {
        element.block_movement && element.pos() == (x, y)
    })
}

pub fn move_by(id: usize, map: &Map, elements: &mut [Element], dx: i32, dy: i32) {
    let (x, y) = elements[id].pos();
    if !is_blocked(x + dx, y + dy, map, elements) {
        elements[id].set_pos(x + dx, y + dy);
    }
}
