use crate::core::{
    components::{
        base_component::BaseComponent,
        properties::{overflow::Overflow, size::SizePolicy},
    },
    utils::traits::renderable::Renderable,
};

pub struct Container {
    base: BaseComponent,
    children: Vec<Box<dyn Renderable>>,
    overflow: Overflow,
    sizing_policy: SizePolicy,
}
