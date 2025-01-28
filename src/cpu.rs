use crate::memory::Memory;

pub struct CPU {
    memory: Memory,
    register: [u8; 16],
    pc: u16,
    i: u16,
    delay_timer: u8,
    sound_timer: u8
} 

impl CPU {
    pub fn new() -> Self {
        CPU {
            memory: Memory::new(),
            register: [0; 16],
            pc: 0x200,
            i: 0,
            delay_timer: 0,
            sound_timer: 0
        }
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        self.memory.read(address)
    }

    pub fn execute(&mut self) {
        while self.pc < 0xFFF {
            let first_byte = self.read_memory(self.pc);
            let second_byte = self.read_memory(self.pc + 1);

            self.pc += 2;
        }
    }

    pub fn parse_instruction(&mut self) {}
}