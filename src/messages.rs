use tcod::colors::Color;
use crate::constants::MSG_HEIGHT;

pub type Messages = Vec<(String, Color)>;

pub trait MessageLog {
    fn add<T: Into<String>>(&mut self, message: T, color: Color);
}

impl MessageLog for Messages {
    fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        while self.len() >= MSG_HEIGHT {
            self.remove(0);
        }
        self.push((message.into(), color))
    }
}
