use crate::{
    colors::*,
    constants::PLAYER,
    position::Position,
};
use tcod::colors::{Color};

#[derive(Debug)]
pub struct Element {
    pub position: Position,
    pub char: char,
    pub color: Color,
    pub block_movement: bool,
    pub alive: bool,
    pub display_name: String,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
    pub item: Option<Item>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
}

impl Element {
    pub fn new(x: i32, y: i32,
               display_name: &str,
               char: char, color: Color,
               block_movement: bool) -> Self {
        Element {
            position: Position::new(x, y),
            display_name: display_name.to_owned(),
            char: char,
            color: color,
            block_movement: block_movement,
            alive: false,
            fighter: None,
            ai: None,
            item: None,
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn distance_to(&self, other: &Element) -> f32 {
        self.position.distance_to(&other.position)
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

    pub fn attack(&mut self, target: &mut Element) {
        let damage = self.fighter.map_or(0, |f| f.strength) -
            target.fighter.map_or(0, |f| f.defense);
        if damage > 0 {
            println!("{} attacks {} for {} hit points", self.display_name, target.display_name, damage);
            target.take_damage(damage);
        } else {
            println!("{} attacks {} but it has no effect!", self.display_name, target.display_name);
        }
    }

    /// heal by the given amount, without going over the maximum
    pub fn heal(&mut self, amount: i32) {
        if let Some(ref mut fighter) = self.fighter {
            fighter.hp += amount;
            if fighter.hp > fighter.max_hp {
                fighter.hp = fighter.max_hp;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub strength: i32,
    on_death: DeathCallback,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    fn callback(self, element: &mut Element) {
        use DeathCallback::*;
        let callback: fn(&mut Element) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(element);
    }
}

fn player_death(player: &mut Element) {
    println!("You died!");
    player.char = '%';
    player.color = COLOR_PLAYER_DEAD;
}

fn monster_death(monster: &mut Element) {
    println!("{} is dead!", monster.display_name);
    monster.char = '%';
    monster.color = COLOR_MONSTER_DEAD;
    monster.block_movement = false;
    monster.fighter = None;
    monster.ai = None;
    monster.display_name = format!("remains of {}", monster.display_name);
}

pub fn make_orc(x: i32, y: i32) -> Element {
    let mut orc = Element::new(x, y, "orc", 'o', COLOR_MONSTER_ORC, true);
    orc.fighter = Some(Fighter{
        max_hp: 10,
        hp: 10,
        defense: 0,
        strength: 3,
        on_death: DeathCallback::Monster,
    });
    orc.ai = Some(Ai);
    orc.alive = true;
    orc
}

pub fn make_troll(x: i32, y: i32) -> Element {
    let mut troll = Element::new(x, y, "troll", 'T', COLOR_MONSTER_TROLL, true);
    troll.fighter = Some(Fighter{
        max_hp: 10,
        hp: 10,
        defense:1,
        strength: 4,
        on_death: DeathCallback::Monster,
    });
    troll.ai = Some(Ai);
    troll.alive = true;
    troll
}

pub fn make_player(x: i32, y: i32) -> Element {
    let mut player = Element::new(x, y, "player", '@', COLOR_PLAYER, true);
    player.alive = true;
    player.fighter = Some(Fighter{
        max_hp: 30,
        hp: 30,
        defense: 2,
        strength: 5,
        on_death: DeathCallback::Player,
    });
    player
}

/// add to the player's inventory and remove from the map
pub fn pick_item_up(object_id: usize, elements: &mut Vec<Element>, inventory: &mut Vec<Element>) {
    // use tcod::colors;
    const MAX_INVENTORY_ITEMS : u32 = 26;
    if inventory.len() as u32 >= MAX_INVENTORY_ITEMS {
        // message(messages,
        //         format!("Your inventory is full, cannot pick up {}.", elements[object_id].display_name),
        //         colors::RED);
    } else {
        let item = elements.swap_remove(object_id);
        // message(messages, format!("You picked up a {}!", item.display_name), colors::GREEN);
        inventory.push(item);
    }
}

enum UseResult {
    UsedUp,
    Cancelled,
}


pub fn drop_item(inventory_id: usize,
                 inventory: &mut Vec<Element>,
                 elements: &mut Vec<Element>) {
    let mut item = inventory.remove(inventory_id);
    item.set_pos(elements[PLAYER].position.x, elements[PLAYER].position.y);
    // message(messages, format!("You dropped a {}.", item.name), colors::YELLOW);
    elements.push(item);
}

pub fn use_item(inventory_id: usize,
                inventory: &mut Vec<Element>,
                element: &mut [Element]) {
    use Item::*;
    // just call the "use_function" if it is defined
    if let Some(item) = inventory[inventory_id].item {
        let on_use = match item {
            Heal => cast_heal,
        };
        match on_use(inventory_id, element) {
            UseResult::UsedUp => {
                // destroy after use, unless it was cancelled for some reason
                inventory.remove(inventory_id);
            }
            UseResult::Cancelled => {
                // message(messages, "Cancelled", colors::WHITE);
            }
        }
    } else {
        // message(messages,
        //         format!("The {} cannot be used.", inventory[inventory_id].name),
        //         colors::WHITE);
    }
}

fn cast_heal(_inventory_id: usize, elements: &mut [Element]) -> UseResult {
    // heal the player
    if let Some(fighter) = elements[PLAYER].fighter {
        if fighter.hp == fighter.max_hp {
            return UseResult::Cancelled;
        }
        elements[PLAYER].heal(5);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}

pub fn make_potion(x: i32, y: i32) -> Element {
    let mut potion = Element::new(x, y, "potion", '!', COLOR_POTION, false);
    potion.item = Some(Item::Heal);
    potion
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ai;
