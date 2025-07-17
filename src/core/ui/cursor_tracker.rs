use crate::core::components::properties::position::Position;

#[derive(Debug, PartialEq, Eq)]
pub struct CursorTracker {
    cursor_position: Position,
}

impl CursorTracker {
    pub fn update_position(&self) {
        // call windows API for cursor tracking
    }

    pub fn current_position(&self) -> Position {
        Position::default()
    }
}
