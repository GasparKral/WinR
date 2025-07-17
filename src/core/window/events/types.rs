#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    // Keyboard events
    KeyPressed,
    KeyReleased,
    KeyTyped,
    KeyHeld,
    KeyCombinationPressed,
    KeyCombinationReleased,
    KeyCombinationHeld,
    KeyCombinationTyped,

    // Mouse events
    MouseButtonPressed,
    MouseButtonReleased,
    MouseButtonClicked,
    MouseButtonDoubleClicked,
    MouseMoved,
    MouseScrolled,

    // Animation events
    AnimationStarted,
    AnimationStopped,
    AnimationPaused,
    AnimationResumed,
    AnimationFrameUpdated,
    AnimationCompleted,
    AnimationTransitionStarted,
    AnimationTransitionEnded,

    // Form events
    FieldFocused,
    FieldFocusedLost,
    FieldValueChanged,
    FieldValidating,
    FieldFocusMove,

    // System events
    RenderRequested,
    UpdateRequested,
}
