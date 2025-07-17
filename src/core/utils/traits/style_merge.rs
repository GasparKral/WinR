pub trait StyleMerge {
    /// Combina este estilo con otro, donde `other` tiene precedencia sobre `self`
    fn merge_with(&self, other: &Self) -> Self;
}

use crate::core::components::styles::base_styles::BaseStyles;
use crate::core::components::properties::graphics::{background::Background, border::Border, color::Color};

impl StyleMerge for BaseStyles {
    fn merge_with(&self, other: &Self) -> Self {
        Self {
            scale: if other.scale != 1.0 { other.scale } else { self.scale },
            background: other.background.clone(),
            border: other.border.clone().or(self.border.clone()),
            text_color: other.text_color,
            z_index: other.z_index,
        }
    }
}

impl Default for BaseStyles {
    fn default() -> Self {
        Self {
            scale: 1.0,
            background: Background::Solid(Color::rgb(255, 255, 255)),
            border: None,
            text_color: Color::rgb(0, 0, 0),
            z_index: 0,
        }
    }
}
