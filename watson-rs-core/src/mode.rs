use derive_more::TryFrom;

#[derive(TryFrom, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[try_from(repr)]
#[repr(u8)]
pub enum Mode {
    A = 0x1,
    S = 0x2,
}

impl<E> From<Mode> for Result<Mode, E> {
    fn from(value: Mode) -> Self {
        Ok(value)
    }
}

impl Mode {
    pub const MODES_COUNT: usize = 2;

    pub fn next_mode(self) -> Mode {
        let state_machine = [
            Mode::S, // A = 1 -> S
            Mode::A, // S = 2 -> A
        ];
        state_machine[self as usize - 1]
    }

    pub fn modes() -> &'static [Mode] {
        &[Mode::A, Mode::S]
    }
}

#[cfg(test)]
mod unit_tests {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    fn from_u8() {
        assert!(matches!(Mode::try_from(0x1), Ok(Mode::A)));
        assert!(matches!(Mode::try_from(0x2), Ok(Mode::S)));
    }

    #[test]
    fn to_u8() {
        assert_eq!(Mode::A as u8, 0x1);
        assert_eq!(Mode::S as u8, 0x2);
    }

    #[test]
    fn switch_mode() {
        let mode = Mode::A;
        assert_eq!(mode, Mode::A);

        let mode = mode.next_mode();
        assert_eq!(mode, Mode::S);

        let mode = mode.next_mode();
        assert_eq!(mode, Mode::A);
    }

    #[test]
    fn modes_list() {
        assert_eq!(Mode::modes(), &[Mode::A, Mode::S]);
    }
}
