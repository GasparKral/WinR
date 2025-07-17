use crate::core::{
    components::{base_component::BaseComponent, text_component::TextComponent},
    utils::traits::renderable::Renderable,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    base: BaseComponent,
    text: TextComponent,
}
