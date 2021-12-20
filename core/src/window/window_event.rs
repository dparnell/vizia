use crate::Entity;
use crate::{CursorIcon, MouseButton};

use keyboard_types::{Code, Key};
use morphorm::GeometryChanged;

// // Used by the GeometryChanged event to signal that either posx, posy, width, or height of the entity have changed
// #[derive(Debug, Default, Clone, PartialEq)]
// pub struct GeometryChanged {
//     pub posx: bool,
//     pub posy: bool,
//     pub width: bool,
//     pub height: bool,
// }

// Events generated by the application in response to OS events
// Or events that can be used to set properties of the window
#[derive(Debug, Clone)]
pub enum WindowEvent {
    /// Emitted when a window is closed
    WindowClose,
    /// Emitted when a window is opened
    WindowResize(f32, f32),
    /// Emitted when a mouse button is double clicked
    MouseDoubleClick(MouseButton),
    /// Emitted when a mouse button is pressed
    MouseDown(MouseButton),
    /// Emitted when a mouse button is released
    MouseUp(MouseButton),
    /// Emitted when the mouse cursor is moved
    MouseMove(f32, f32),
    /// Emitted when the mouse scroll wheel is scrolled
    MouseScroll(f32, f32),
    /// Emitted when the mouse cursor enters the bounding box of an entity
    MouseOver,
    /// Emitted when the mouse cursor leaves the bounding box of an entity
    MouseOut,
    /// Emitted when the mouse cursor enters an entity or one of its decendants
    MouseEnter,
    /// Emitted when the mouse cursor leaves an entity or one of its decendants
    MouseLeave,

    FocusIn,

    FocusOut,

    /// Emitted when a character is typed
    CharInput(char),
    /// Emitted when a keyboard key is pressed
    KeyDown(Code, Option<Key>),
    /// Emitted when a keyboard key is released
    KeyUp(Code, Option<Key>),
    /// Sets the mouse cursor icon
    SetCursor(CursorIcon),
    /// Grabs the mouse cursor, preventing it from leaving the window
    GrabCursor(bool),
    /// Sets the (x,y) position of the mouse cursor in window coordinates
    SetCursorPosition(u32, u32),
    /// Emitted when mouse events have been captured
    MouseCaptureEvent,
    /// Emitted when mouse events have been released
    MouseCaptureOutEvent,
    /// Emitted when an entity changes position or size (TODO: check if this includes margins + borders)
    GeometryChanged(GeometryChanged),
    /// Requests a redraw of the window contents
    Redraw,
    /// Request a restyle
    Restyle,
    /// Requests a relayout
    Relayout,
    /// Prints the debug message to the console
    Debug(String),
    //
    // // TODO - move to WidgetEvent
    // ChildAdded(Entity),
}
