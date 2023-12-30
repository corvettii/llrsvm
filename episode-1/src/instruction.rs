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

impl From<Instruction> for u8 {
    fn from(orig: Instruction) -> u8 {
        orig as u8
    }
}
