mod cpu;
use cpu::Cpu;

mod instruction;
use instruction::Instruction;

mod memory;
use memory::Memory;

fn main() {
    let mut memory = Memory::new(256);
    let mut address: usize = 0;

    memory.write_byte(address, Instruction::MovLitR1.into());
    address += 1;
    memory.write_word(address, 0x1234);
    address += 2;

    memory.write_byte(address, Instruction::MovLitR2.into());
    address += 1;
    memory.write_word(address, 0xabcd);
    address += 2;

    memory.write_byte(address, Instruction::AddRegReg.into());
    address += 1;
    memory.write_byte(address, Cpu::get_register_offset("r1"));
    address += 1;
    memory.write_byte(address, Cpu::get_register_offset("r2"));
    
    let mut cpu = Cpu::new(memory);
    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();
}