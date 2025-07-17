use crate::core::components::styles::component_style::ComponentStyle;

pub trait Renderable {
    /// Renderiza el componente con el estilo inyectado
    fn render(&mut self, style: &ComponentStyle, context: &mut dyn RenderContext);
    
    /// Retorna el tipo de componente para resolución de estilos
    fn component_type(&self) -> &'static str;
    
    /// Obtiene el estado actual del componente para estilos específicos de estado
    fn get_component_state(&self) -> ComponentState {
        ComponentState::Default
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentState {
    Default,
    Hover,
    Pressed,
    Disabled,
    Focused,
}

/// Context de renderizado que debe ser implementado por el sistema de renderizado
pub trait RenderContext {
    fn draw_background(&mut self, x: f32, y: f32, width: f32, height: f32, background: &crate::core::components::properties::graphics::background::Background);
    fn draw_border(&mut self, x: f32, y: f32, width: f32, height: f32, border: &crate::core::components::properties::graphics::border::Border);
    fn draw_text(&mut self, text: &str, x: f32, y: f32, color: &crate::core::components::properties::graphics::color::Color, font_size: f32);
    fn set_opacity(&mut self, opacity: f32);
    fn push_clip_rect(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn pop_clip_rect(&mut self);
}
