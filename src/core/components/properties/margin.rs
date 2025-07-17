use crate::core::utils::traits::observable::Observable;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Margin {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Margin {
    pub fn new(top: u16, right: u16, bottom: u16, left: u16) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn horizontal(&self) -> u16 {
        self.left + self.right
    }

    pub fn vertical(&self) -> u16 {
        self.top + self.bottom
    }
}

impl Observable<Margin> for Margin {
    fn subscribe<F>(&mut self, callback: F)
    where
        F: FnMut(&Margin) + 'static,
    {
        todo!()
    }

    fn notify(&self, value: &Margin) {
        todo!()
    }

    fn unsubscribe<F>(&mut self, callback: F)
    where
        F: FnMut(&Margin) + 'static,
    {
        todo!()
    }

    fn clear_subscriptions(&mut self) {
        todo!()
    }

    fn has_subscriptions(&self) -> bool {
        todo!()
    }
}
