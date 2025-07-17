use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA { r, g, b, a }
    }

    pub fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        RGBA { r, g, b, a }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }

    pub fn from((r, g, b): (u8, u8, u8)) -> Self {
        RGB { r, g, b }
    }
    pub fn to_rgba(self, a: u8) -> RGBA {
        RGBA::new(self.r, self.g, self.b, a)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HSV {
    pub h: u8,
    pub s: u8,
    pub v: u8,
}

impl HSV {
    pub fn new(h: u8, s: u8, v: u8) -> Self {
        HSV { h, s, v }
    }

    pub fn from((h, s, v): (u8, u8, u8)) -> Self {
        HSV { h, s, v }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HEX {
    pub value: &'static str,
}

impl HEX {
    pub fn new(value: &'static str) -> Self {
        if !Self::validate_format(value) {
            panic!("Invalid HEX color format: {}", value);
        }
        HEX { value }
    }

    fn validate_format(value: &str) -> bool {
        // Simple validation for hex format (e.g., "#RRGGBB" or "#RGB" or "#RRGGBBAA" or "0xRRGGBB" or "0xRGB" or "0xRRGGBBAA")
        let re = Regex::new(r"^(#|0x)?([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$").unwrap();
        re.is_match(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    RGBA(RGBA),
    RGB(RGB),
    HSV(HSV),
    HEX(HEX),
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::RGBA(RGBA::new(r, g, b, a))
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::RGB(RGB::new(r, g, b))
    }

    pub fn hsv(h: u8, s: u8, v: u8) -> Self {
        Color::HSV(HSV::new(h, s, v))
    }

    pub fn hex(value: &'static str) -> Self {
        Color::HEX(HEX::new(value))
    }
}
