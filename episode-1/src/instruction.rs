pub enum Instruction {
    MovLitR1 = 0x10,
    MovLitR2 = 0x11,
    AddRegReg = 0x12,
}

impl From<u8> for Instruction {
    fn from(orig: u8) -> Self {
        match orig {
            0x10 => Self::MovLitR1,
            0x11 => Self::MovLitR2,
            0x12 => Self::AddRegReg,
            _ => panic!(),
        }
    }
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        self as u8
    }
}
