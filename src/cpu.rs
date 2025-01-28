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
            let first_byte = (self.read_memory(self.pc) as u16) << 8;
            let second_byte = self.read_memory(self.pc + 1) as u16;

            let instruction = first_byte & second_byte;
            self.parse_instruction(instruction);

            self.pc += 2;
        }
    }

    pub fn parse_instruction(&mut self, instruction: u16) {
        let opcode = instruction & 0b1111_0000_0000_0000;

        match opcode {
            0x0 => {},
            0x1000 => {},
            0x2000 => {},
            0x3000 => {},
            0x4000 => {},
            0x5000 => {},
            0x6000 => {
                let constant = (instruction & 0x00FF) as u8;
                let register_index = ((instruction & 0x0F00) >> 8) as usize;

                self.register[register_index] = constant;
            },
            0x7000 => {
                let constant = (instruction & 0x00FF) as u8;
                let register_index = ((instruction & 0x0F00) >> 8) as usize;

                self.register[register_index]+= constant; 
            },
            0x8000 => {
                match instruction & 0x000F {
                    0 => {
                        let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                        let register_y_index = ((instruction & 0x00F0) >> 4) as usize;

                        self.register[register_x_index] = self.register[register_y_index];
                    },
                    1 => {
                        let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                        let register_y_index = ((instruction & 0x00F0) >> 4) as usize;

                        self.register[register_x_index] = self.register[register_x_index] | self.register[register_y_index];
                    },
                    2 => {
                        let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                        let register_y_index = ((instruction & 0x00F0) >> 4) as usize;

                        self.register[register_x_index] = self.register[register_x_index] & self.register[register_y_index];
                    },
                    3 => {
                        let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                        let register_y_index = ((instruction & 0x00F0) >> 4) as usize;

                        self.register[register_x_index] = self.register[register_x_index] ^ self.register[register_y_index];
                    },
                    4 => {
                        let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                        let register_y_index = ((instruction & 0x00F0) >> 4) as usize;

                        if (self.register[register_x_index] as u16) + (self.register[register_y_index] as u16) > 0xFF {
                            self.register[15] = 1;
                        } else {
                            self.register[15] = 0;
                        }

                        self.register[register_x_index] = self.register[register_x_index] + self.register[register_y_index];
                    },
                    5 => {
                        let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                        let register_y_index = ((instruction & 0x00F0) >> 4) as usize;

                        if (self.register[register_x_index] as i16) - (self.register[register_y_index] as i16) < 0x0 {
                            self.register[15] = 0;
                        } else {
                            self.register[15] = 1;
                        }

                        self.register[register_x_index] = self.register[register_x_index] - self.register[register_y_index];
                    },
                    6 => {
                        
                    },
                    7 => {},
                    0xE => {},
                    _ => {}
                }
                
            },
            0x9000 => {},
            0xA000 => {},
            0xB000 => {},
            0xC000 => {},
            0xD000 => {},
            0xE000 => {},
            0xF000 => {},
            _ => {}
        }
    }
}