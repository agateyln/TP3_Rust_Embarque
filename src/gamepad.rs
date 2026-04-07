use core::convert::Infallible;
use embedded_hal::digital::InputPin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    Center,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, defmt::Format)]
pub struct GamepadState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub center: bool,
}

pub struct Gamepad<UP, DOWN, LEFT, RIGHT, CENTER> {
    up: UP,
    down: DOWN,
    left: LEFT,
    right: RIGHT,
    center: CENTER,
}

impl<UP, DOWN, LEFT, RIGHT, CENTER> Gamepad<UP, DOWN, LEFT, RIGHT, CENTER>
where
    UP: InputPin<Error = Infallible>,
    DOWN: InputPin<Error = Infallible>,
    LEFT: InputPin<Error = Infallible>,
    RIGHT: InputPin<Error = Infallible>,
    CENTER: InputPin<Error = Infallible>,
{
    pub fn new(up: UP, down: DOWN, left: LEFT, right: RIGHT, center: CENTER) -> Self {
        Self {
            up,
            down,
            left,
            right,
            center,
        }
    }

    pub fn is_pressed(&mut self, button: Button) -> bool {
        match button {
            Button::Up => self.up.is_low().unwrap_or(false),
            Button::Down => self.down.is_low().unwrap_or(false),
            Button::Left => self.left.is_low().unwrap_or(false),
            Button::Right => self.right.is_low().unwrap_or(false),
            Button::Center => self.center.is_low().unwrap_or(false),
        }
    }

    pub fn poll(&mut self) -> GamepadState {
        GamepadState {
            up: self.is_pressed(Button::Up),
            down: self.is_pressed(Button::Down),
            left: self.is_pressed(Button::Left),
            right: self.is_pressed(Button::Right),
            center: self.is_pressed(Button::Center),
        }
    }
}
