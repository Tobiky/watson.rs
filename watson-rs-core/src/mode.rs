#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Mode {
    A = 0x1,
    S = 0x2,
}

impl From<i32> for Mode {
    fn from(integer: i32) -> Self {
        match integer {
            0x1 => Mode::A,
            0x2 => Mode::S,
            _ => panic!("tried to convert non-supported integer to Mode"),
        }
    }
}

impl From<Mode> for i32 {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::A => 0x1,
            Mode::S => 0x2,
        }
    }
}

impl Mode {
    pub const MODES_COUNT: usize = 2;

    pub fn next_mode(self) -> Mode {
        let mode = i32::from(self);
        let new_mode = (mode << 1) % (Self::MODES_COUNT as i32 + 1);
        Mode::from(new_mode)
    }

    pub fn modes() -> Vec<Mode> {
        vec![Mode::A, Mode::S]
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn from_i32() {
        assert_eq!(Mode::from(0x1), Mode::A);
        assert_eq!(Mode::from(0x2), Mode::S);
    }

    #[test]
    fn to_i32() {
        assert_eq!(i32::from(Mode::A), 0x1);
        assert_eq!(i32::from(Mode::S), 0x2);
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
    fn modes_vector() {
        assert_eq!(Mode::modes(), vec![Mode::A, Mode::S]);
    }
}
