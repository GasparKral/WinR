use crate::core::ui::{elements::theme::Theme, systems::theme_system::ThemeSystem};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct LayoutSystem {
    theme: Theme,
}

impl LayoutSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(&mut self) {
        self.theme = ThemeSystem::load_theme();
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }
}
