use crate::{
    colors,
    constants::{MAP_HEIGHT, MAP_WIDTH, PLAYER},
    element::{Element, make_player},
    fov::FovMap,
    map::{Map, generate_map},
    messages::{Messages,MessageLog},
};

pub struct GameState {
    pub elements: Vec<Element>,
    pub map: Map,
    pub inventory: Vec<Element>,
    pub fov_map: FovMap,
    pub log: Messages,
}

pub fn new_game() -> GameState {
    let player = make_player(0, 0);
    let mut elements: Vec<Element> = vec![player];
    let inventory : Vec<Element> = vec![];
    let mut log : Messages = vec![];

    let (map, starting_position) = generate_map(&mut elements);
    elements[PLAYER].set_pos(starting_position.0, starting_position.1);

    let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            fov_map.set(x, y, map[x as usize][y as usize].block_sight);
        }
    }

    log.add("Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.",
            colors::COLOR_PLAYER_DEAD); // TODO color
    GameState {
        elements: elements,
        map: map,
        inventory: inventory,
        fov_map: fov_map,
        log: log,
    }
}
