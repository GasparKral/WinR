use crate::core::utils::traits::observable::Observable;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

impl Observable<Position> for Position {
    fn subscribe<F>(&mut self, callback: F)
    where
        F: FnMut(&Position) + 'static,
    {
        todo!()
    }

    fn notify(&self, value: &Position) {
        todo!()
    }

    fn unsubscribe<F>(&mut self, callback: F)
    where
        F: FnMut(&Position) + 'static,
    {
        todo!()
    }

    fn clear_subscriptions(&mut self) {
        todo!()
    }

    fn has_subscriptions(&self) -> bool {
        todo!()
    }
}
