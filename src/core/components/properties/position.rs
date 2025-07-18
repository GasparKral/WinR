use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Position {
        Position { x, y }
    }

    pub fn in_x(mut self, x: u16) -> Position {
        self.x = x;
        self
    }

    pub fn in_y(mut self, y: u16) -> Position {
        self.y = y;
        self
    }

    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
}
