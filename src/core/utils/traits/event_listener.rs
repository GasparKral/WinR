use crate::core::window::events::types::Event;

pub trait EventListener {
    fn on_event(&mut self, event: &Event, caller_id: usize);
}
