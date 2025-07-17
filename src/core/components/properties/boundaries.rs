use crate::core::{components::properties::position::Position, ui::cursor_tracker::CursorTracker};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Boundaries {
    p0: Position,
    p1: Position,
    p2: Position,
    p3: Position,
    // CORNER POSITION ON CLOCKWISE ORDER
    //   p0-------------p1
    // 	 |               |
    //   |               |
    //   p3-------------p2
}

impl Boundaries {
    pub fn new(p0: Position, p1: Position, p2: Position, p3: Position) -> Self {
        Self { p0, p1, p2, p3 }
    }

    pub fn inside_boundary(&self, cursor: CursorTracker) -> bool {
        let cursor_pos = cursor.current_position();

        cursor_pos.x() >= self.p0.x()
            && cursor_pos.x() <= self.p2.x()
            && cursor_pos.y() >= self.p0.y()
            && cursor_pos.y() <= self.p2.y()
    }
}
