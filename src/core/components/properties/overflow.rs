#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
    Scroll,
    Auto,
}

impl Overflow {
    pub fn as_str(&self) -> &str {
        match self {
            Overflow::Visible => "visible",
            Overflow::Hidden => "hidden",
            Overflow::Scroll => "scroll",
            Overflow::Auto => "auto",
        }
    }
}
