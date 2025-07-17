use std::{fmt::Debug, hash::Hash};

use crate::core::components::styles::button::ButtonStyle;

pub enum IconPosition {
    Start,
    End,
}

pub struct Button {
    /*ICON: Bitmap/SVG/Image */
    text: String,
    enabled: bool,
    style: ButtonStyle,
    on_click: Option<Box<dyn Fn()>>,
}

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Implement Debug for Button");
    }
}

impl Hash for Button {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!("Implement Hash for Button");
    }
}
