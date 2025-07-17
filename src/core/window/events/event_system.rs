use std::{cell::RefCell, collections::HashMap, rc::Weak};

use crate::core::{utils::traits::event_listener::EventListener, window::events::types::Event};

#[derive(Debug, Clone, Default)]
pub struct EventSystem {
    listeners: HashMap<Event, Vec<Weak<RefCell<dyn EventListener>>>>,
    next_id: usize,
}

impl EventSystem {
    pub fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn subscribe<L: EventListener + 'static>(
        &mut self,
        event: Event,
        listener: Weak<RefCell<L>>,
    ) {
        let listener = listener as Weak<RefCell<dyn EventListener>>;
        self.listeners.entry(event).or_default().push(listener);
    }

    pub fn emit(&mut self, event: Event, caller_id: usize) {
        if let Some(listeners) = self.listeners.get_mut(&event) {
            // Filtrar listeners que ya no existen y notificar a los válidos
            listeners.retain(|weak_listener| {
                if let Some(listener) = weak_listener.upgrade() {
                    listener.borrow_mut().on_event(&event, caller_id);
                    true
                } else {
                    false // Remover listeners muertos
                }
            });
        }
    }

    // Limpiar listeners muertos periódicamente
    pub fn cleanup(&mut self) {
        for listeners in self.listeners.values_mut() {
            listeners.retain(|weak_listener| weak_listener.strong_count() > 0);
        }
    }
}
