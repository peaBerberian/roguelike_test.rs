use crate::constants::PLAYER;
use tcod::input::{
    Key,
    KeyCode,
    Mouse,
};
use crate::element::*;
use crate::fov::FovMap;
use crate::map::{
    Map,
    move_by,
};
use crate::utils::mut_two;
use crate::render::*;
use crate::state::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

/// return a string with the names of all elements under the mouse
pub fn get_names_under_mouse(mouse: Mouse, elements: &[Element], fov_map: &FovMap) -> Vec<String> {
    let (x, y) = (mouse.cx as i32, mouse.cy as i32);

    // create a list with the names of all elements at the mouse's coordinates and in FOV
    elements
        .iter()
        .filter(|elt| {
            elt.pos() == (x, y) &&
            fov_map.is_in_sight(elt.position.x, elt.position.y)
        })
        .map(|elt| elt.display_name.clone())
        .collect::<Vec<_>>()
}

fn player_move_or_attack(map: &Map, elements: &mut [Element], dx: i32, dy: i32) {
    let (x, y) = elements[PLAYER].pos();
    let new_x = x + dx;
    let new_y = y + dy;

    let target_id = elements.iter().position(|element| {
        element.fighter.is_some() && element.pos() == (new_x, new_y)
    });

    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER, target_id, elements);
            player.attack(target);
        }
        None => {
            move_by(PLAYER, map, elements, dx, dy);
        }
    }
}

pub fn handle_keys_player_mode(
    root: &mut tcod::console::Root,
    key: Key,
    game_state: &mut GameState,
) -> PlayerAction {
    use PlayerAction::*;

    match key {
        // NSWE
        Key { code: KeyCode::Up, .. } | Key { code: KeyCode::NumPad8, .. } => {
            player_move_or_attack(&game_state.map, &mut game_state.elements, 0, -1);
            TookTurn
        }
        Key { code: KeyCode::Down, .. } | Key { code: KeyCode::NumPad2, .. } => {
            player_move_or_attack(&game_state.map, &mut game_state.elements, 0, 1);
            TookTurn
        }
        Key { code: KeyCode::Left, .. } | Key { code: KeyCode::NumPad4, .. } => {
            player_move_or_attack(&game_state.map, &mut game_state.elements, -1, 0);
            TookTurn
        }
        Key { code: KeyCode::Right, .. } | Key { code: KeyCode::NumPad6, .. } => {
            player_move_or_attack(&game_state.map, &mut game_state.elements, 1, 0);
            TookTurn
        }

        // Diagonals
        Key { code: KeyCode::NumPad7, .. }=> {
            player_move_or_attack(&game_state.map, &mut game_state.elements, -1, -1);
            TookTurn
        }
        Key { code: KeyCode::NumPad9, .. } => {
            player_move_or_attack(&game_state.map, &mut game_state.elements, 1, -1);
            TookTurn
        }
        Key { code: KeyCode::NumPad1, .. } => {
            player_move_or_attack(&game_state.map, &mut game_state.elements, -1, 1);
            TookTurn
        }
        Key { code: KeyCode::NumPad3, .. } => {
            player_move_or_attack(&game_state.map, &mut game_state.elements, 1, 1);
            TookTurn
        }

        Key { printable: 'g', .. } => {
            let item_id = game_state.elements.iter().position(|elt| {
                elt.pos() == game_state.elements[PLAYER].pos() && elt.item.is_some()
            });
            if let Some(item_id) = item_id {
                pick_item_up(item_id, &mut game_state.elements, &mut game_state.inventory);
            }
            DidntTakeTurn
        }

        Key { printable: 'i', .. } => {
            // show the inventory
            let inventory_index = inventory_menu(
                &game_state.inventory,
                "Press the key next to an item to use it, or any other to cancel.\n",
                root);
            if let Some(inventory_index) = inventory_index {
                use_item(inventory_index, &mut game_state.inventory, &mut game_state.elements);
            }
            DidntTakeTurn
        }

        Key { printable: 'd', .. } => {
            // show the inventory; if an item is selected, drop it
            let inventory_index = inventory_menu(&game_state.inventory,
                                                 "Press the key next to an item to drop it,
                                                 or any other to cancel.\n'",
                                                 root);
            if let Some(inventory_index) = inventory_index {
                drop_item(inventory_index, &mut game_state.inventory, &mut game_state.elements);
            }
            DidntTakeTurn
        }

        Key { code: KeyCode::Escape, .. } => Exit,
        _ => DidntTakeTurn,
    }
}

pub fn handle_keys_dead_mode(key: Key) -> PlayerAction {
    use PlayerAction::*;

    match key {
        Key { code: KeyCode::Escape, .. } => Exit,
        _ => DidntTakeTurn,
    }
}
