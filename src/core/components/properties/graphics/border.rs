use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::core::components::properties::graphics::color::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderType {
    Solid,
    Dashed,
    Dotted,
    Double,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Border {
    color: Color,
    width: f32,
    border_type: BorderType,
}

impl Eq for Border {}

impl Hash for Border {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.width.to_bits().hash(state);
        self.border_type.hash(state);
    }
}

impl Border {
    pub fn new(color: Color, width: f32, border_type: BorderType) -> Self {
        Self {
            color,
            width,
            border_type,
        }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn border_type(&self) -> &BorderType {
        &self.border_type
    }
}
