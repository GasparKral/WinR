use std::{fmt::Debug, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::core::components::{
    base_component::BaseComponent, elements::icon::Icon, properties::position::Position,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum IconPosition {
    Start,
    End,
    Located(Position),
}

#[derive(Serialize, Deserialize)]
pub struct Button {
    base: BaseComponent,
    icon: Option<(Icon, IconPosition)>,
    text: String,
    enabled: bool,
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
