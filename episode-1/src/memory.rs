#[derive(Debug)]

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0; size],
        }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn read_word(&self, address: usize) -> u16 {
        u16::from_ne_bytes([self.memory[address], self.memory[address + 1]])
    }

    pub fn write_byte(&mut self, address: usize, byte: u8) {
        self.memory[address] = byte;
    }

    pub fn write_word(&mut self, address: usize, word: u16) {
        self.write_byte(address, ((word & 0xff) as u8).try_into().unwrap());
        self.write_byte(address + 1, (word >> 8 as u8).try_into().unwrap());
    }
}