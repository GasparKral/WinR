#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Padding {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Padding {
    pub fn new(top: u16, right: u16, bottom: u16, left: u16) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn horizontal(&self, horizontal: u16) -> Self {
        Self {
            top: self.top,
            right: horizontal / 2,
            bottom: self.bottom,
            left: horizontal / 2,
        }
    }

    pub fn vertical(&self, vertical: u16) -> Self {
        Self {
            top: vertical / 2,
            right: self.right,
            bottom: vertical / 2,
            left: self.left,
        }
    }

    pub fn top(&self, top: u16) -> Self {
        Self {
            top,
            right: self.right,
            bottom: self.bottom,
            left: self.left,
        }
    }

    pub fn right(&self, right: u16) -> Self {
        Self {
            top: self.top,
            right,
            bottom: self.bottom,
            left: self.left,
        }
    }

    pub fn bottom(&self, bottom: u16) -> Self {
        Self {
            top: self.top,
            right: self.right,
            bottom,
            left: self.left,
        }
    }

    pub fn left(&self, left: u16) -> Self {
        Self {
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            left,
        }
    }
}
