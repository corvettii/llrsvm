use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::instruction::Instruction;
use crate::memory::Memory;

const REGISTER_NAMES: &[&str] = &[
    "ip", "acc", "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8",
];

lazy_static! {
    static ref REGISTER_MAP: HashMap<&'static str, usize> = REGISTER_NAMES
        .iter()
        .enumerate()
        .map(|(index, &element)| (element, index * 2))
        .collect();
}

#[derive(Debug)]
pub struct Cpu {
    memory: Memory,
    registers: Memory,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            registers: Memory::new(REGISTER_NAMES.len() * 2),
        }
    }

    pub fn debug(&self) {
        REGISTER_NAMES
            .iter()
            .for_each(|name| println!("{: >3}: 0x{:04x}", name, self.get_register(name)));
        println!();
    }

    fn get_register(&self, name: &str) -> u16 {
        self.registers.read_word(REGISTER_MAP[name])
    }

    pub fn get_register_offset(name: &str) -> u8 {
        (REGISTER_MAP[name] / 2).try_into().unwrap()
    }

    fn set_register(&mut self, name: &str, value: u16) {
        self.registers.write_word(REGISTER_MAP[name], value);
    }

    fn fetch_byte(&mut self) -> u8 {
        let instruction_address: usize = self.get_register("ip").into();
        let instruction = self.memory.read_byte(instruction_address);

        self.set_register("ip", (instruction_address + 1).try_into().unwrap());

        return instruction;
    }

    fn fetch_word(&mut self) -> u16 {
        let instruction_address: usize = self.get_register("ip").into();
        let instruction = self.memory.read_word(instruction_address);

        self.set_register("ip", (instruction_address + 2).try_into().unwrap());

        return instruction;
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::MovLitR1 => {
                let literal = self.fetch_word();
                self.set_register("r1", literal);
            }
            Instruction::MovLitR2 => {
                let literal = self.fetch_word();
                self.set_register("r2", literal);
            }
            Instruction::AddRegReg => {
                let r1 = (self.fetch_byte() as usize) * 2;
                let r2 = (self.fetch_byte() as usize) * 2;
                let v1 = self.registers.read_word(r1);
                let v2 = self.registers.read_word(r2);
                self.set_register("acc", v1 + v2);
            }
        }
    }

    pub fn step(&mut self) {
        let instruction: Instruction = self.fetch_byte().into();
        self.execute(instruction);
    }
}
