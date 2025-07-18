use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionState {
    #[default]
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

impl ActionState {
    pub fn is_normal(&self) -> bool {
        matches!(self, ActionState::Normal)
    }

    pub fn is_hovered(&self) -> bool {
        matches!(self, ActionState::Hovered)
    }

    pub fn is_pressed(&self) -> bool {
        matches!(self, ActionState::Pressed)
    }

    pub fn is_disabled(&self) -> bool {
        matches!(self, ActionState::Disabled)
    }
}
