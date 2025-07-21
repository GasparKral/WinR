use std::{cell::RefCell, hash::Hash, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::core::{
    components::properties::{
        boundaries::Boundaries, margin::Margin, padding::Padding, position::Position, size::Size,
    },
    window::events::{event_system::EventSystem, types::EventType},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoundarySizingMode {
    ContentBox,
    #[default]
    BorderBox,
    MarginBox,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WrapMode {
    NoWrap,
    #[default]
    Wrap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseComponent {
    #[serde(skip)]
    id: usize,
    size: Size,
    position: Position,
    #[serde(skip)]
    bounds: Option<Boundaries>,
    margin: Margin,
    padding: Padding,
    visible: bool,
    wrap_mode: WrapMode,
    sizing_mode: BoundarySizingMode,
    #[serde(skip)]
    event_system: Rc<RefCell<EventSystem>>,
}

impl PartialEq for BaseComponent {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
            && self.position == other.position
            && self.bounds == other.bounds
            && self.margin == other.margin
            && self.padding == other.padding
            && self.visible == other.visible
            && self.sizing_mode == other.sizing_mode
    }
}

impl Eq for BaseComponent {}

impl Hash for BaseComponent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.position.hash(state);
        self.bounds.hash(state);
        self.margin.hash(state);
        self.padding.hash(state);
        self.visible.hash(state);
        self.sizing_mode.hash(state);
    }
}

impl BaseComponent {
    pub fn new(
        size: Size,
        position: Position,
        margin: Margin,
        padding: Padding,
        event_system: Rc<RefCell<EventSystem>>,
    ) -> Self {
        let id = event_system.borrow_mut().get_next_id();

        let bound_p0_x = position.x() + padding.left;
        let bound_p0_y = position.y() + padding.top;

        let bound_p1_x = position.x() + size.width() - padding.right;
        let bound_p1_y = position.y() + padding.top;

        let bound_p2_x = position.x() + size.width() - padding.right;
        let bound_p2_y = position.y() + size.height() - padding.bottom;

        let bound_p3_x = position.x() + padding.left;
        let bound_p3_y = position.y() + size.height() - padding.bottom;

        let bounds = Boundaries::new(
            Position::new(bound_p0_x, bound_p0_y),
            Position::new(bound_p1_x, bound_p1_y),
            Position::new(bound_p2_x, bound_p2_y),
            Position::new(bound_p3_x, bound_p3_y),
        );

        Self {
            id,
            size,
            position,
            bounds: Some(bounds),
            margin,
            padding,
            visible: true,
            wrap_mode: WrapMode::default(),
            sizing_mode: BoundarySizingMode::default(),
            event_system,
        }
    }

    pub fn calculate_bounds(&mut self) {
        match self.sizing_mode {
            BoundarySizingMode::ContentBox => self.calculate_content_box_bounds(),
            BoundarySizingMode::BorderBox => self.calculate_border_box_bounds(),
            BoundarySizingMode::MarginBox => self.calculate_margin_box_bounds(),
        }
    }

    fn calculate_margin_box_bounds(&mut self) {
        // Use Margin and Padding for boundaries
        let bound_p0_x = self.position.x() + self.margin.left + self.padding.left;
        let bound_p0_y = self.position.y() + self.margin.top + self.padding.top;
        let bound_p1_x =
            self.position.x() + self.size.width() - self.margin.right - self.padding.right;
        let bound_p1_y = self.position.y() + self.margin.top + self.padding.top;
        let bound_p2_x =
            self.position.x() + self.size.width() - self.margin.right - self.padding.right;
        let bound_p2_y =
            self.position.y() + self.size.height() - self.margin.bottom - self.padding.bottom;
        let bound_p3_x = self.position.x() + self.margin.left + self.padding.left;
        let bound_p3_y =
            self.position.y() + self.size.height() - self.margin.bottom - self.padding.bottom;

        self.bounds = Some(Boundaries::new(
            Position::new(bound_p0_x, bound_p0_y),
            Position::new(bound_p1_x, bound_p1_y),
            Position::new(bound_p2_x, bound_p2_y),
            Position::new(bound_p3_x, bound_p3_y),
        ));
    }

    fn calculate_border_box_bounds(&mut self) {
        let bound_p0_x = self.position.x() + self.padding.left;
        let bound_p0_y = self.position.y() + self.padding.top;

        let bound_p1_x = self.position.x() + self.size.width() - self.padding.right;
        let bound_p1_y = self.position.y() + self.padding.top;

        let bound_p2_x = self.position.x() + self.size.width() - self.padding.right;
        let bound_p2_y = self.position.y() + self.size.height() - self.padding.bottom;

        let bound_p3_x = self.position.x() + self.padding.left;
        let bound_p3_y = self.position.y() + self.size.height() - self.padding.bottom;

        self.bounds = Some(Boundaries::new(
            Position::new(bound_p0_x, bound_p0_y),
            Position::new(bound_p1_x, bound_p1_y),
            Position::new(bound_p2_x, bound_p2_y),
            Position::new(bound_p3_x, bound_p3_y),
        ));
    }

    fn calculate_content_box_bounds(&mut self) {
        // Igore margin and padding for border box
        let bound_p0_x = self.position.x();
        let bound_p0_y = self.position.y();
        let bound_p1_x = self.position.x() + self.size.width();
        let bound_p1_y = self.position.y();
        let bound_p2_x = self.position.x() + self.size.width();
        let bound_p2_y = self.position.y() + self.size.height();
        let bound_p3_x = self.position.x();
        let bound_p3_y = self.position.y() + self.size.height();

        self.bounds = Some(Boundaries::new(
            Position::new(bound_p0_x, bound_p0_y),
            Position::new(bound_p1_x, bound_p1_y),
            Position::new(bound_p2_x, bound_p2_y),
            Position::new(bound_p3_x, bound_p3_y),
        ));
    }

    pub fn inyect_event_system(&mut self, event_system: Rc<RefCell<EventSystem>>) {
        self.id = event_system.borrow_mut().get_next_id();
        self.event_system = event_system;
    }

    // Métodos que emiten eventos
    pub fn set_size(&mut self, size: Size) {
        if self.size != size {
            self.size = size;
            self.invalidate_bounds();
            self.event_system
                .borrow_mut()
                .emit(EventType::ComponentResized, self.id);
        }
    }

    pub fn set_position(&mut self, position: Position) {
        if self.position != position {
            self.position = position;
            self.invalidate_bounds();
            self.event_system
                .borrow_mut()
                .emit(EventType::ComponentMoved, self.id);
        }
    }

    pub fn set_margin(&mut self, margin: Margin) {
        if self.margin != margin {
            self.margin = margin;
            self.invalidate_bounds();
            self.event_system
                .borrow_mut()
                .emit(EventType::ComponentMarginChanged, self.id);
        }
    }

    pub fn set_padding(&mut self, padding: Padding) {
        if self.padding != padding {
            self.padding = padding;
            self.invalidate_bounds();
            self.event_system
                .borrow_mut()
                .emit(EventType::ComponentPaddingChanged, self.id);
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        if self.visible != visible {
            self.visible = visible;
            self.event_system
                .borrow_mut()
                .emit(EventType::ComponentVisibilityChanged, self.id);
        }
    }

    fn invalidate_bounds(&mut self) {
        self.bounds = None;
    }

    // Getters inmutables
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn position(&self) -> &Position {
        &self.position
    }
    pub fn margin(&self) -> &Margin {
        &self.margin
    }
    pub fn padding(&self) -> &Padding {
        &self.padding
    }
    pub fn sizing_mode(&self) -> BoundarySizingMode {
        self.sizing_mode
    }
    pub fn bounds(&self) -> Option<&Boundaries> {
        self.bounds.as_ref()
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn visible(&self) -> &bool {
        &self.visible
    }

    pub fn wrap_mode(&self) -> &WrapMode {
        &self.wrap_mode
    }
}
