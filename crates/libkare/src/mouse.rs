#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum MouseButton {
    Left = 0,
    Right,
    Middle,
}

impl TryFrom<u8> for MouseButton {
    // TODO: Create actual error
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > MouseButton::Middle as u8 {
            return Err(format!("Invalid mouse button value: {}", value));
        }

        /// SAFTY: The bounds of theu value have been checked above, the rest of the values are
        /// valid keys.
        unsafe {
            Ok(std::mem::transmute(value))
        }
    }
}

impl From<MouseButton> for u8 {
    fn from(button: MouseButton) -> u8 {
        button as u8
    }
}

impl From<rdev::Button> for MouseButton {
    fn from(value: rdev::Button) -> Self {
        match value {
            rdev::Button::Left => MouseButton::Left,
            rdev::Button::Right => MouseButton::Right,
            rdev::Button::Middle => MouseButton::Middle,
            rdev::Button::Unknown(_) => todo!(),
        }
    }
}

impl From<MouseButton> for rdev::Button {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => rdev::Button::Left,
            MouseButton::Right => rdev::Button::Right,
            MouseButton::Middle => rdev::Button::Middle,
        }
    }
}
