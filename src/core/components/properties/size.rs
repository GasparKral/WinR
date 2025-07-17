use crate::core::utils::traits::observable::Observable;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    height: u16,
    width: u16,
}

impl Size {
    pub fn new(height: u16, width: u16) -> Size {
        Size { height, width }
    }

    pub fn with_width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}

impl Observable<Size> for Size {
    fn subscribe<F>(&mut self, callback: F)
    where
        F: FnMut(&Size) + 'static,
    {
        todo!()
    }

    fn notify(&self, value: &Size) {
        todo!()
    }

    fn unsubscribe<F>(&mut self, callback: F)
    where
        F: FnMut(&Size) + 'static,
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
