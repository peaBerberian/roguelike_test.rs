extern crate tcod;
extern crate rand;

mod ai;
mod colors;
mod constants;
mod element;
mod fov;
mod input;
mod map;
mod messages;
mod position;
mod render;
mod state;
mod utils;

use crate::{
    // state::{GameState, new_game},
    state::new_game,
    constants::{
        LIMIT_FPS,
        MAP_HEIGHT,
        MAP_WIDTH,
        PANEL_HEIGHT,
        PLAYER,
        SCREEN_HEIGHT,
        SCREEN_WIDTH,
        TORCH_RADIUS,
    },
    input::{
        get_names_under_mouse,
        handle_keys_dead_mode,
        handle_keys_player_mode,
        PlayerAction,
    },
    map::explore,
};
use tcod::console::*;
use tcod::input::{self as tcodInput, Event};

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust roguelike tutorial")
        .init();
    let mut con_map = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let mut panel = Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT);

    let mut mouse = Default::default();
    let mut key = Default::default();

    tcod::system::set_fps(LIMIT_FPS);

    let mut game_state = new_game();
    while !root.window_closed() {
        let player = &game_state.elements[PLAYER];
        game_state.fov_map.compute_fov(player.position.x,
                                       player.position.y,
                                       TORCH_RADIUS);
        explore(&mut game_state.map, &game_state.fov_map);

        match tcodInput::check_for_event(tcodInput::MOUSE | tcodInput::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => mouse = m,
            Some((_, Event::Key(k))) => key = k,
            _ => key = Default::default(),
        }

        let hovered = get_names_under_mouse(mouse,
                                            &game_state.elements,
                                            &game_state.fov_map);
        render::render_all(&mut root,
                           &mut con_map,
                           &mut panel,
                           &hovered,
                           &game_state);

        let player_action = if game_state.elements[PLAYER].alive {
            handle_keys_player_mode(&mut root, key, &mut game_state)
        } else {
            handle_keys_dead_mode(key)
        };

        if player_action == PlayerAction::Exit {
            break;
        }

        // let monsters take their turn
        if game_state.elements[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..game_state.elements.len() {
                if game_state.elements[id].ai.is_some() {
                    ai::ai_take_turn(id, &game_state.map, & mut game_state.elements, PLAYER, &game_state.fov_map);
                }
            }
        }
    }
}
