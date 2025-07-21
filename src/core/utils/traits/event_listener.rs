use crate::core::window::events::types::EventType;

pub trait EventListener {
    fn on_event(&mut self, event: &EventType, caller_id: usize);
}
