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
