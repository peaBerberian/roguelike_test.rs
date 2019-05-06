use tcod::colors::{self, Color};

pub const COLOR_DARK_WALL: Color = Color { r: 10, g: 5, b: 5 };
pub const COLOR_LIGHT_WALL: Color = Color { r: 40, g: 15, b: 15 };
pub const COLOR_DARK_GROUND: Color = Color { r: 50, g: 32, b: 32 };
pub const COLOR_LIGHT_GROUND: Color = Color { r: 60, g: 42, b: 32 };

pub const COLOR_PLAYER: Color = colors::WHITE;
pub const COLOR_PLAYER_DEAD: Color = colors::DARK_RED;

pub const COLOR_MONSTER_ORC: Color = colors::DESATURATED_GREEN;
pub const COLOR_MONSTER_TROLL: Color = colors::DARKER_GREEN;
pub const COLOR_MONSTER_DEAD: Color = colors::DARK_RED;
pub const COLOR_POTION: Color = colors::VIOLET;

pub const COLOR_HP_FOREGROUND: Color = colors::LIGHT_RED;
pub const COLOR_HP_BACKGROUND: Color = colors::DARKER_RED;
