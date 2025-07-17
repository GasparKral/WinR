use crate::core::components::properties::graphics::color::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GradientType {
    Linear(i16),             // Angle in degrees
    Radial((i16, i16), i16), // Center point and radius
    Conic(i16),              // Angle in degrees
}

impl GradientType {
    pub fn is_linear(&self) -> bool {
        matches!(self, GradientType::Linear(_))
    }

    pub fn is_radial(&self) -> bool {
        matches!(self, GradientType::Radial(_, _))
    }

    pub fn is_conic(&self) -> bool {
        matches!(self, GradientType::Conic(_))
    }

    pub fn lienar(angle: i16) -> Self {
        GradientType::Linear(angle)
    }

    pub fn radial(center: (i16, i16), radius: i16) -> Self {
        GradientType::Radial(center, radius)
    }

    pub fn conic(angle: i16) -> Self {
        GradientType::Conic(angle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gradient {
    pub gradient_type: GradientType,
    pub stops: Vec<(Color, u16)>, // Color and position (0-100)
}

impl Gradient {
    pub fn new(gradient_type: GradientType, stops: Vec<(Color, u16)>) -> Self {
        Gradient {
            gradient_type,
            stops,
        }
    }
}
