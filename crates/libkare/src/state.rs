use crate::{key::Key, mouse::MouseButton};

macro_rules! impl_state {
    ($state_name:ident, $iterator_name:ident, $kind:ty, $size:ty) => {
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $state_name($size);

        impl $state_name {
            pub fn with_pressed(values: &[$kind]) -> Self {
                let mut state = Self::default();
                for k in values {
                    state.set_pressed(*k);
                }
                state
            }

            pub fn is_pressed(&self, value: $kind) -> bool {
                (self.0 >> value as u8) as u8 & 1u8 == 1
            }

            pub fn set_pressed(&mut self, value: $kind) {
                self.0 |= 1 << (value as u8);
            }

            pub fn set_released(&mut self, value: $kind) {
                self.0 &= !(1 << (value as u8));
            }

            pub fn is_state_held(&self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }

            pub fn iter(&self) -> $iterator_name {
                $iterator_name::new(self.0)
            }
        }

        pub struct $iterator_name($size);

        impl $iterator_name {
            fn new(state: $size) -> Self {
                Self(state)
            }
        }

        impl Iterator for $iterator_name {
            type Item = $kind;

            fn next(&mut self) -> Option<Self::Item> {
                if self.0 == 0 {
                    return None;
                }
                let index = self.0.trailing_zeros() as u8;
                self.0 &= !(1 << index);
                index.try_into().ok()
            }
        }

        impl IntoIterator for $state_name {
            type Item = $kind;
            type IntoIter = $iterator_name;

            fn into_iter(self) -> Self::IntoIter {
                $iterator_name::new(self.0)
            }
        }
    };
}

impl_state!(KeyState, KeyIterator, Key, u128);
impl_state!(MouseState, MouseIterator, MouseButton, u8);

mod test {
    use super::*;

    #[test]
    fn key_state() {
        let mut state = KeyState::default();
        assert!(!state.is_pressed(Key::A));
        state.set_pressed(Key::A);
        assert!(state.is_pressed(Key::A));
        state.set_released(Key::A);
        assert!(!state.is_pressed(Key::A));
    }

    #[test]
    fn mouse_state() {
        let mut state = MouseState::default();
        assert!(!state.is_pressed(MouseButton::Left));
        state.set_pressed(MouseButton::Left);
        assert!(state.is_pressed(MouseButton::Left));
        state.set_released(MouseButton::Left);
        assert!(!state.is_pressed(MouseButton::Left));
    }

    #[test]
    fn key_state_iter() {
        let mut state = KeyState::default();
        state.set_pressed(Key::A);
        state.set_pressed(Key::B);
        state.set_pressed(Key::C);

        let mut iter = state.iter();
        assert_eq!(iter.next(), Some(Key::A));
        assert_eq!(iter.next(), Some(Key::B));
        assert_eq!(iter.next(), Some(Key::C));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mouse_state_iter() {
        let mut state = MouseState::default();
        state.set_pressed(MouseButton::Left);
        state.set_pressed(MouseButton::Right);
        state.set_pressed(MouseButton::Middle);

        let mut iter = state.iter();
        assert_eq!(iter.next(), Some(MouseButton::Left));
        assert_eq!(iter.next(), Some(MouseButton::Right));
        assert_eq!(iter.next(), Some(MouseButton::Middle));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn key_state_with_pressed() {
        let state = KeyState::with_pressed(&[Key::A, Key::B, Key::C]);
        let mut iter = state.iter();
        assert_eq!(iter.next(), Some(Key::A));
        assert_eq!(iter.next(), Some(Key::B));
        assert_eq!(iter.next(), Some(Key::C));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn key_state_is_state_held() {
        let mut state = KeyState::default();
        state.set_pressed(Key::A);
        state.set_pressed(Key::B);
        state.set_pressed(Key::C);

        let mut other = KeyState::default();
        other.set_pressed(Key::A);
        other.set_pressed(Key::B);

        assert!(state.is_state_held(other));
    }

    #[test]
    fn mouse_state_is_state_held() {
        let mut state = MouseState::default();
        state.set_pressed(MouseButton::Left);
        state.set_pressed(MouseButton::Right);
        state.set_pressed(MouseButton::Middle);

        let mut other = MouseState::default();
        other.set_pressed(MouseButton::Left);
        other.set_pressed(MouseButton::Right);

        assert!(state.is_state_held(other));
    }

    #[test]
    fn key_state_into_iter() {
        let mut state = KeyState::default();
        state.set_pressed(Key::A);
        state.set_pressed(Key::B);
        state.set_pressed(Key::C);

        let mut iter = state.into_iter();
        assert_eq!(iter.next(), Some(Key::A));
        assert_eq!(iter.next(), Some(Key::B));
        assert_eq!(iter.next(), Some(Key::C));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mouse_state_into_iter() {
        let mut state = MouseState::default();
        state.set_pressed(MouseButton::Left);
        state.set_pressed(MouseButton::Right);
        state.set_pressed(MouseButton::Middle);

        let mut iter = state.into_iter();
        assert_eq!(iter.next(), Some(MouseButton::Left));
        assert_eq!(iter.next(), Some(MouseButton::Right));
        assert_eq!(iter.next(), Some(MouseButton::Middle));
        assert_eq!(iter.next(), None);
    }
}
