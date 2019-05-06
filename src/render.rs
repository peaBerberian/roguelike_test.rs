use crate::colors::*;
use crate::constants::{
    BAR_WIDTH,
    INVENTORY_WIDTH,
    MAP_HEIGHT,
    MAP_WIDTH,
    PANEL_HEIGHT,
    PANEL_Y,
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    MSG_HEIGHT,
    MSG_WIDTH,
    MSG_X,
    PLAYER,
};
use crate::{
    state::GameState,
    element::Element,
};
use tcod::colors::{self, Color};
use tcod::console::*;

pub fn render_all(
    root: &mut Root,
    con: &mut Offscreen,
    panel: &mut Offscreen,
    hovered: &Vec<String>,
    game_state: &GameState
) {
    con.set_default_foreground(colors::WHITE);
    con.clear();

    let mut to_draw: Vec<_> = game_state.elements
        .iter()
        .filter(|e| game_state.fov_map.is_in_sight(e.position.x, e.position.y))
        .collect();

    // sort so that non-blocknig objects come first
    to_draw.sort_by(|e1, e2| { e1.block_movement.cmp(&e2.block_movement) });
    // draw the objects in the list
    for element in &to_draw {
        con.set_default_foreground(element.color);
        con.put_char(element.position.x,
                     element.position.y,
                     element.char,
                     BackgroundFlag::None);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let explored = game_state.map[x as usize][y as usize].explored;
            if explored {
                let is_visible = game_state.fov_map.is_in_sight(x, y);
                let is_wall = game_state.map[x as usize][y as usize].block_sight;
                let color = match (is_visible, is_wall) {
                    (false, true) => COLOR_DARK_WALL,
                    (false, false) => COLOR_DARK_GROUND,
                    (true, true) => COLOR_LIGHT_WALL,
                    (true, false) => COLOR_LIGHT_GROUND,
                };
                con.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }
    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);

    panel.set_default_background(colors::BLACK);
    panel.clear();

    let hp = game_state.elements[PLAYER].fighter.map_or(0, |f| f.hp);
    let max_hp = game_state.elements[PLAYER].fighter.map_or(0, |f| f.max_hp);
    render_bar(panel, 1, 1, BAR_WIDTH, "HP", hp, max_hp, COLOR_HP_FOREGROUND, COLOR_HP_BACKGROUND);

    // print the game messages, one line at a time
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in game_state.log.iter().rev() {
        let msg_height = panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        panel.set_default_foreground(color);
        panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    panel.set_default_foreground(colors::LIGHT_GREY);
    panel.print_ex(1, 0, BackgroundFlag::None, TextAlignment::Left, hovered.join(", "));

    // blit the contents of `panel` to the root console
    blit(panel, (0, 0), (SCREEN_WIDTH, PANEL_HEIGHT), root, (0, PANEL_Y), 1.0, 1.0);

    root.flush();
}

pub fn inventory_menu(inventory: &[Element], header: &str, root: &mut Root) -> Option<usize> {
    // how a menu with each item of the inventory as an option
    let options = if inventory.len() == 0 {
        vec!["Inventory is empty.".into()]
    } else {
        inventory.iter().map(|item| { item.display_name.clone() }).collect()
    };

    let inventory_index = menu(header, &options, INVENTORY_WIDTH, root);

    // if an item was chosen, return it
    if inventory.len() > 0 {
        inventory_index
    } else {
        None
    }
}

fn menu<T: AsRef<str>>(header: &str, options: &[T], width: i32,
                           root: &mut Root) -> Option<usize> {
    assert!(options.len() <= 26, "Cannot have a menu with more than 26 options.");

    // calculate total height for the header (after auto-wrap) and one line per option
    let header_height = root.get_height_rect(0, 0, width, SCREEN_HEIGHT, header);
    let height = options.len() as i32 + header_height;

    // create an off-screen console that represents the menu's window
    let mut window = Offscreen::new(width, height);

    // print the header, with auto-wrap
    window.set_default_foreground(colors::WHITE);
    window.print_rect_ex(0, 0, width, height, BackgroundFlag::None, TextAlignment::Left, header);

    // print all the options
    for (index, option_text) in options.iter().enumerate() {
        let menu_letter = (b'a' + index as u8) as char;
        let text = format!("({}) {}", menu_letter, option_text.as_ref());
        window.print_ex(0, header_height + index as i32,
                        BackgroundFlag::None, TextAlignment::Left, text);
    }


    // blit the contents of "window" to the root console
    let x = SCREEN_WIDTH / 2 - width / 2;
    let y = SCREEN_HEIGHT / 2 - height / 2;
    tcod::console::blit(&mut window, (0, 0), (width, height), root, (x, y), 1.0, 0.7);

    // present the root console to the player and wait for a key-press
    root.flush();
    let key = root.wait_for_keypress(true);

    // convert the ASCII code to an index; if it corresponds to an option, return it
    if key.printable.is_alphabetic() {
        let index = key.printable.to_ascii_lowercase() as usize - 'a' as usize;
        if index < options.len() {
            Some(index)
        } else {
            None
        }
    } else {
        None
    }
}

fn render_bar(panel: &mut Offscreen,
              x: i32,
              y: i32,
              total_width: i32,
              name: &str,
              value: i32,
              maximum: i32,
              bar_color: Color,
              back_color: Color)
{
    // render a bar (HP, experience, etc). First calculate the width of the bar
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

    // render the background first
    panel.set_default_background(back_color);
    panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

    // now render the bar on top
    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }

    // finally, some centered text with the values
    panel.set_default_foreground(colors::WHITE);
    panel.print_ex(x + total_width / 2, y, BackgroundFlag::None, TextAlignment::Center,
                   &format!("{}: {}/{}", name, value, maximum));
}
