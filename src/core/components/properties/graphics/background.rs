use crate::core::components::properties::graphics::{color::Color, gradient::Gradient};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BackgroundColor {
    Solid(Color),
    Gradient(Gradient),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BackgroundShape {
    Rectangle,
    RoundedRectangle { radius: f32 },
}

impl Eq for BackgroundShape {}
impl Hash for BackgroundShape {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            BackgroundShape::Rectangle => "Rectangle".hash(state),
            BackgroundShape::RoundedRectangle { radius } => {
                "RoundedRectangle".hash(state);
                radius.to_bits().hash(state);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Background {
    color: BackgroundColor,
    shape: BackgroundShape,
}

impl Background {
    pub fn new_solid(color: Color, shape: BackgroundShape) -> Self {
        Self {
            color: BackgroundColor::Solid(color),
            shape,
        }
    }

    pub fn new_gradient(gradient: Gradient, shape: BackgroundShape) -> Self {
        Self {
            color: BackgroundColor::Gradient(gradient),
            shape,
        }
    }

    pub fn color(&self) -> &BackgroundColor {
        &self.color
    }

    pub fn shape(&self) -> &BackgroundShape {
        &self.shape
    }
}
