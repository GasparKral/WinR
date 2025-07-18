# WinR - Rust UI Library

## Table of Contents

1. [Overview](#overview)
2. [Project Architecture](#project-architecture)
3. [Installation and Setup](#installation-and-setup)
4. [Core Components](#core-components)
5. [Event System](#event-system)
6. [Properties and Styling](#properties-and-styling)
7. [Usage Guide](#usage-guide)
8. [Testing](#testing)
9. [API Reference](#api-reference)
10. [Contributing](#contributing)

## Overview

WinR is a Rust library designed for developing Windows-style GUI applications. The library provides a modular, component-oriented framework that enables the creation of robust user interfaces for Windows applications.

### Key Features

-   **Component-oriented architecture**: Modular system based on reusable components
-   **Advanced event system**: Event handling with weak references to prevent memory leaks
-   **CSS-like properties**: Sizing, positioning, margin and padding system similar to CSS
-   **Serialization**: Complete support for serialization/deserialization with Serde
-   **Integrated testing**: Complete testing framework for components and serialization
-   **Windows API Integration**: Native integration with Windows API

### Technologies and Dependencies

-   **Rust Edition 2024**
-   **Serde**: For serialization and deserialization
-   **Windows API**: For native Windows integration
-   **Regex**: For text processing and validations

## Project Architecture

The project is organized into clearly defined modules:

```
src/
├── core/                    # Library core
│   ├── components/          # Component system
│   │   ├── base_component.rs
│   │   ├── elements/        # UI Elements (Button, Icon, Link)
│   │   ├── layouts/         # Layout systems
│   │   ├── properties/      # Component properties
│   │   └── styles/          # Style system
│   ├── ui/                  # User interface system
│   │   ├── elements/        # Advanced UI elements
│   │   └── systems/         # UI systems (layout, theme, navigation)
│   ├── utils/               # Utilities
│   │   ├── functions.rs
│   │   └── traits/          # Fundamental traits
│   └── window/              # Window and event system
│       └── events/
├── testing/                 # Testing framework
│   ├── blackbox/
│   ├── run/
│   └── serialization/
└── lib.rs
```

## Installation and Setup

### System Requirements

-   Rust 1.75+ (Edition 2024)
-   Windows 10/11
-   Visual Studio Build Tools (for Windows API)

### Adding WinR to your project

```toml
[dependencies]
WinR = { path = "path/to/WinR" }
```

### Included dependencies

```toml
[dependencies]
regex = "1.11.1"
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
windows = "0.61.3"
```

## Core Components

### BaseComponent

The `BaseComponent` is the base class for all components in WinR. It provides fundamental functionalities such as:

#### Key Features

-   **Unique ID**: Each component has a unique identifier
-   **Positioning system**: Position, Size, Margin, Padding
-   **Calculated boundaries**: Automatic bounds based on sizing mode
-   **Event system**: Integration with EventSystem
-   **Serialization**: Complete Serde support

#### Sizing Modes

```rust
pub enum BoundarySizingMode {
    ContentBox,    // Without margin or padding
    BorderBox,     // Includes padding, excludes margin
    MarginBox,     // Includes margin and padding
}
```

#### Usage Example

```rust
use WinR::core::components::base_component::BaseComponent;
use WinR::core::components::properties::{Size, Position, Margin, Padding};

let size = Size::new(200, 100);
let position = Position::new(10, 20);
let margin = Margin::uniform(5);
let padding = Padding::uniform(10);

let mut component = BaseComponent::new(
    size,
    position,
    margin,
    padding,
    event_system
);
```

### UI Elements

#### Button

Button component with icon support and click events.

```rust
pub struct Button {
    icon: Option<(Icon, IconPosition)>,
    text: String,
    enabled: bool,
    on_click: Option<Box<dyn Fn()>>,
}
```

**Features:**

-   Customizable text
-   Optional icons (Start/End position)
-   Enabled/disabled state
-   Click callback

#### Icon

Integrated icon system for UI elements.

#### Link

Link component with navigation.

## Event System

### EventSystem

The event system uses weak references to prevent memory leaks and provides a robust pub/sub system.

#### Event Types

```rust
pub enum Event {
    // Component events
    ComponentAdded,
    ComponentRemoved,
    ComponentResized,
    ComponentMoved,
    ComponentPaddingChanged,
    ComponentMarginChanged,
    ComponentVisibilityChanged,

    // Window events
    WindowOpened,
    WindowClosed,
    WindowFocused,
    WindowMaximized,
    WindowMinimized,
    WindowResized,
    WindowMoved,
    WindowTitleChanged,
    WindowIconChanged,

    // Input events
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,

    // Animation events
    AnimationStarted,
    AnimationCompleted,
    AnimationTransitionStarted,
    AnimationTransitionEnded,

    // Form events
    FieldFocused,
    FieldFocusedLost,
    FieldValueChanged,

    // System events
    RenderRequested,
    UpdateRequested,
}
```

#### EventListener Implementation

```rust
use WinR::core::utils::traits::event_listener::EventListener;
use WinR::core::window::events::types::Event;

struct MyComponent {
    // ... fields
}

impl EventListener for MyComponent {
    fn on_event(&mut self, event: &Event, caller_id: usize) {
        match event {
            Event::ComponentResized => {
                // Handle resize
            },
            Event::MouseButtonPressed => {
                // Handle mouse click
            },
            _ => {}
        }
    }
}
```

#### Event Subscription

```rust
// Subscribe a component to events
event_system.borrow_mut().subscribe(
    Event::ComponentResized,
    Rc::downgrade(&component_rc)
);

// Emit an event
event_system.borrow_mut().emit(Event::ComponentResized, component_id);
```

## Properties and Styling

### Property System

#### Size

```rust
pub struct Size {
    height: u16,
    width: u16,
}

// Usage
let size = Size::new(200, 100)
    .with_width(250)
    .with_height(150);
```

#### Position

```rust
pub struct Position {
    x: u16,
    y: u16,
}

// Usage
let position = Position::new(10, 20)
    .in_x(50)
    .in_y(100);
```

#### Margin and Padding

```rust
// Uniform margin
let margin = Margin::uniform(10);

// Individual margin
let margin = Margin {
    top: 5,
    right: 10,
    bottom: 5,
    left: 10,
};
```

### Graphics Properties

The `graphics` module includes:

-   **ActionState**: Interaction states (Normal, Hover, Pressed, Disabled)
-   **Background**: Background configuration
-   **Border**: Border system
-   **Color**: Color handling
-   **Gradient**: Linear and radial gradients

### Boundaries

The boundaries system automatically calculates component bounds based on:

-   Component position
-   Component size
-   Applied margin
-   Applied padding
-   Selected BoundarySizingMode

## Usage Guide

### Creating Basic Components

```rust
use std::{cell::RefCell, rc::Rc};
use WinR::core::{
    components::{
        base_component::BaseComponent,
        properties::{Size, Position, Margin, Padding}
    },
    window::events::event_system::EventSystem
};

fn create_basic_component() -> BaseComponent {
    let event_system = Rc::new(RefCell::new(EventSystem::default()));

    let component = BaseComponent::new(
        Size::new(200, 100),
        Position::new(10, 10),
        Margin::uniform(5),
        Padding::uniform(10),
        event_system
    );

    component
}
```

### Event Handling

```rust
// Create event system
let event_system = Rc::new(RefCell::new(EventSystem::default()));

// Create component
let mut component = BaseComponent::new(
    Size::new(100, 50),
    Position::new(0, 0),
    Margin::default(),
    Padding::default(),
    event_system.clone()
);

// Events are automatically emitted when changing properties
component.set_size(Size::new(150, 75)); // Emits Event::ComponentResized
component.set_position(Position::new(20, 30)); // Emits Event::ComponentMoved
```

### Serialization

```rust
use serde_json;

// Serialize component
let component = create_basic_component();
let json = serde_json::to_string(&component).unwrap();

// Deserialize component
let mut deserialized: BaseComponent = serde_json::from_str(&json).unwrap();

// Re-inject event system after deserializing
deserialized.inyect_event_system(event_system);
```

## Testing

WinR includes a complete testing framework organized into three modules:

### Serialization Tests

Located in `src/testing/serialization/`, includes tests for:

-   **background_tests.rs**: Background properties testing
-   **border_tests.rs**: Border properties testing
-   **color_tests.rs**: Color system testing
-   **gradient_tests.rs**: Gradient testing
-   **margin_tests.rs**: Margin testing
-   **overflow_tests.rs**: Overflow testing
-   **padding_tests.rs**: Padding testing
-   **position_tests.rs**: Positioning testing
-   **size_tests.rs**: Size testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific serialization tests
cargo test serialization

# Run tests with detailed output
cargo test -- --nocapture
```

### Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let event_system = Rc::new(RefCell::new(EventSystem::default()));
        let component = BaseComponent::new(
            Size::new(100, 50),
            Position::new(10, 20),
            Margin::uniform(5),
            Padding::uniform(8),
            event_system
        );

        assert_eq!(component.size().width(), 100);
        assert_eq!(component.size().height(), 50);
        assert_eq!(component.position().x(), 10);
        assert_eq!(component.position().y(), 20);
    }
}
```

## API Reference

### BaseComponent Methods

#### Constructors

-   `new(size, position, margin, padding, event_system) -> Self`

#### Mutators (Emit events)

-   `set_size(size: Size)` - Emits `ComponentResized`
-   `set_position(position: Position)` - Emits `ComponentMoved`
-   `set_margin(margin: Margin)` - Emits `ComponentMarginChanged`
-   `set_padding(padding: Padding)` - Emits `ComponentPaddingChanged`
-   `set_visible(visible: bool)` - Emits `ComponentVisibilityChanged`

#### Getters

-   `size() -> &Size`
-   `position() -> &Position`
-   `margin() -> &Margin`
-   `padding() -> &Padding`
-   `bounds() -> Option<&Boundaries>`
-   `id() -> &usize`
-   `visible() -> &bool`
-   `sizing_mode() -> BoundarySizingMode`

#### Utilities

-   `calculate_bounds()` - Recalculates boundaries based on sizing_mode
-   `inyect_event_system(event_system)` - Re-injects the event system

### EventSystem Methods

-   `get_next_id() -> usize` - Gets the next unique ID
-   `subscribe<L: EventListener>(event: Event, listener: Weak<RefCell<L>>)` - Subscribes a listener
-   `emit(event: Event, caller_id: usize)` - Emits an event
-   `cleanup()` - Cleans up removed listeners

### Core Traits

#### EventListener

```rust
pub trait EventListener {
    fn on_event(&mut self, event: &Event, caller_id: usize);
}
```

#### Renderable (Future trait)

```rust
// Located in utils/traits/renderable.rs
// For implementing component rendering
```

#### StyleMerge (Future trait)

```rust
// Located in utils/traits/style_merge.rs
// For CSS-like style merging
```

## Implemented Design Patterns

### Observer Pattern

-   Implemented through EventSystem
-   Weak references prevent memory leaks
-   Automatic cleanup of dead listeners

### Component Pattern

-   BaseComponent as common base
-   Specialization through specific elements
-   Composition over inheritance

### Builder Pattern

-   Chainable methods in Size, Position
-   Fluent component construction

### Serialization Pattern

-   Serde integration for persistence
-   Skip non-serializable fields (event_system)
-   Re-injection pattern for restoring references

## Performance Considerations

### Memory Management

-   Use of `Rc<RefCell<>>` for shared ownership
-   Weak references in event system
-   Automatic listener cleanup

### Event System

-   Automatic filtering of dead listeners
-   Manual cleanup available
-   Unique IDs for efficient identification

### Boundary Calculation

-   Lazy boundary calculation
-   Automatic invalidation when changing properties
-   Different sizing modes for optimization

## Roadmap and Future Features

### Upcoming Features

-   [ ] Complete theme system
-   [ ] Animations and transitions
-   [ ] Advanced layout managers
-   [ ] More UI elements (TextBox, ComboBox, etc.)
-   [ ] Data binding system
-   [ ] Custom drawing support

### Planned Improvements

-   [ ] Event system optimization
-   [ ] Better Windows API integration
-   [ ] Interactive documentation
-   [ ] More complete examples

## Contributing

### Contribution Structure

1. Fork the repository
2. Create feature branch: `git checkout -b feature/new-feature`
3. Write tests for new functionality
4. Implement the functionality
5. Ensure all tests pass: `cargo test`
6. Commit with descriptive messages
7. Push to branch: `git push origin feature/new-feature`
8. Create Pull Request

### Code Standards

-   Follow Rust conventions (rustfmt)
-   Document all public APIs
-   Include tests for new functionality
-   Maintain backward compatibility when possible

### Contribution Areas

-   **New UI components**: Expand the element library
-   **Optimizations**: Improve event system performance
-   **Documentation**: Improve examples and guides
-   **Testing**: Expand test coverage
-   **Windows Integration**: Improve native integration

## License

This project is under the Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International (CC BY-NC-SA 4.0) License. See the `License.md` file for more details.

## Support and Community

-   **Issues**: Report bugs and request features on GitHub
-   **Discussions**: For questions and general discussions
-   **Documentation**: This documentation is updated regularly

---

_Documentation automatically generated for WinR v0.1.0_
