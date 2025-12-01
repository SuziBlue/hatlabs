
use crate::{comms::Source, components::Component};
use crate::geometry::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputDevice {
    Keyboard,
    Mouse,
    Gamepad,
    Touchscreen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputAction {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum InputCode<Pos: Arithmetic, Dir: Arithmetic> {
    Key {
        button: char,
        action: InputAction,
    },
    KeyCode {
        button: u32,
        action: InputAction,
    },
    MouseButton {
        button: u8,
        action: InputAction,
        position: Vec2<Pos>,
    },
    MousePosition(Vec2<Pos>),
    Scroll(Vec2<Dir>),
    GamepadButton {
        button: u8,
        action: InputAction,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UserInputEvent<Pos: Arithmetic, Dir: Arithmetic> {
    pub device: InputDevice,
    pub code: InputCode<Pos, Dir>,
    pub timestamp: u128,
}



pub trait InputHandler<Pos: Arithmetic, Dir: Arithmetic>: Component + Source<UserInputEvent<Pos, Dir>> {}
