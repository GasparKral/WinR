use crate::core::{
    components::properties::{
        boundaries::Boundaries, margin::Margin, padding::Padding, position::Position, size::Size,
    },
    utils::traits::observable::Observable,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BoundarySizingMode {
    ContentBox,
    #[default]
    BorderBox,
    MarginBox,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseComponent {
    size: Size,
    position: Position,
    bounds: Boundaries,
    margin: Margin,
    padding: Padding,
    sizing_mode: BoundarySizingMode,
}

impl BaseComponent {
    pub fn new(size: Size, position: Position, margin: Margin, padding: Padding) -> BaseComponent {
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

        let mut component = BaseComponent {
            size,
            position,
            bounds,
            margin,
            padding,
            sizing_mode: BoundarySizingMode::default(),
        };

        &component.size.subscribe(|_| component.calculate_bounds());
        &component.position.subscribe(|_| component.calculate_bounds());
        &component.margin.subscribe(|_| component.calculate_bounds());
        &component.padding.subscribe(|_| component.calculate_bounds());

        component
    }

    fn calculate_bounds(&mut self) {
        match self.sizing_mode {
            BoundarySizingMode::ContentBox => {
                // Igore margin and padding for border box
                let bound_p0_x = self.position.x();
                let bound_p0_y = self.position.y();
                let bound_p1_x = self.position.x() + self.size.width();
                let bound_p1_y = self.position.y();
                let bound_p2_x = self.position.x() + self.size.width();
                let bound_p2_y = self.position.y() + self.size.height();
                let bound_p3_x = self.position.x();
                let bound_p3_y = self.position.y() + self.size.height();

                self.bounds = Boundaries::new(
                    Position::new(bound_p0_x, bound_p0_y),
                    Position::new(bound_p1_x, bound_p1_y),
                    Position::new(bound_p2_x, bound_p2_y),
                    Position::new(bound_p3_x, bound_p3_y),
                );
            }
            BoundarySizingMode::BorderBox => {
                let bound_p0_x = self.position.x() + self.padding.left;
                let bound_p0_y = self.position.y() + self.padding.top;

                let bound_p1_x = self.position.x() + self.size.width() - self.padding.right;
                let bound_p1_y = self.position.y() + self.padding.top;

                let bound_p2_x = self.position.x() + self.size.width() - self.padding.right;
                let bound_p2_y = self.position.y() + self.size.height() - self.padding.bottom;

                let bound_p3_x = self.position.x() + self.padding.left;
                let bound_p3_y = self.position.y() + self.size.height() - self.padding.bottom;

                self.bounds = Boundaries::new(
                    Position::new(bound_p0_x, bound_p0_y),
                    Position::new(bound_p1_x, bound_p1_y),
                    Position::new(bound_p2_x, bound_p2_y),
                    Position::new(bound_p3_x, bound_p3_y),
                );
            }
            BoundarySizingMode::MarginBox => {
                // Use Margin and Padding for boundaries
                let bound_p0_x = self.position.x() + self.margin.left + self.padding.left;
                let bound_p0_y = self.position.y() + self.margin.top + self.padding.top;
                let bound_p1_x =
                    self.position.x() + self.size.width() - self.margin.right - self.padding.right;
                let bound_p1_y = self.position.y() + self.margin.top + self.padding.top;
                let bound_p2_x =
                    self.position.x() + self.size.width() - self.margin.right - self.padding.right;
                let bound_p2_y = self.position.y() + self.size.height()
                    - self.margin.bottom
                    - self.padding.bottom;
                let bound_p3_x = self.position.x() + self.margin.left + self.padding.left;
                let bound_p3_y = self.position.y() + self.size.height()
                    - self.margin.bottom
                    - self.padding.bottom;

                self.bounds = Boundaries::new(
                    Position::new(bound_p0_x, bound_p0_y),
                    Position::new(bound_p1_x, bound_p1_y),
                    Position::new(bound_p2_x, bound_p2_y),
                    Position::new(bound_p3_x, bound_p3_y),
                );
            }
        }
    }
}
