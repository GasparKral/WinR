use crate::core::components::{
    properties::graphics::{background::Background, border::Border, color::Color},
    styles::base_styles::BaseStyles,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ButtonStyle {
    base_styles: BaseStyles,

    background_hover: Option<Background>,
    background_pressed: Option<Background>,
    background_disabled: Option<Background>,

    border_hover: Option<Border>,
    border_pressed: Option<Border>,
    border_disabled: Option<Border>,

    text_color_hover: Color,
    text_color_pressed: Color,
    text_color_disabled: Color,
}
