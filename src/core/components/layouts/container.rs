use crate::core::{
    components::{
        base_component::BaseComponent,
        properties::{overflow::Overflow, size::SizePolicy},
        styles::base_styles::BaseStyles,
    },
    utils::traits::renderable::Renderable,
};

pub struct Container {
    base: BaseComponent,
    style: BaseStyles,
    children: Vec<Box<dyn Renderable>>,
    overflow: Overflow,
    sizing_policy: SizePolicy,
}
