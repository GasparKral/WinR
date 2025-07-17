#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FontType {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextComponent {
    font_size: u32,
    font_family: String,
    font_type: FontType,
    text: String,
}
