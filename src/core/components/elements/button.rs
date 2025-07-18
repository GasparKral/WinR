use std::{fmt::Debug, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::core::components::elements::icon::Icon;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum IconPosition {
    Start,
    End,
}

#[derive(Serialize, Deserialize)]
pub struct Button {
    icon: Option<(Icon, IconPosition)>,
    text: String,
    enabled: bool,
    #[serde(skip)]
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
