use std::hash::Hash;

use crate::core::components::properties::graphics::{
    background::Background, border::Border, color::Color,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BaseStyles {
    pub scale: f32,
    pub background: Background,
    pub border: Option<Border>,
    pub text_color: Color,
    pub z_index: i32,
}

impl Eq for BaseStyles {}

impl Hash for BaseStyles {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.scale.to_bits().hash(state);
        self.background.hash(state);
        if let Some(border) = &self.border {
            border.hash(state);
        }
        self.text_color.hash(state);
        self.z_index.hash(state);
    }
}
