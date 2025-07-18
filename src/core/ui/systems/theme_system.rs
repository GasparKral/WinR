use crate::core::ui::elements::theme::Theme;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ThemeSystem {}

impl ThemeSystem {
    pub fn load_theme() -> Theme {
        ThemeSystem::default_theme()
    }

    fn default_theme() -> Theme {
        Theme::default()
    }
}
