#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x: x, y: y}
    }

    pub fn distance_to(&self, other_pos: &Position) -> f32 {
        let dx = other_pos.x - self.x;
        let dy = other_pos.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }
}
