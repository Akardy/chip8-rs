use crate::memory::Memory;

pub struct CPU {
    memory: Memory,
    register: [u8; 16],
    pc: u16,
    i: u16,
    stack: [u16; 16],
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
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0
        }
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        self.memory.read(address)
    }

    pub fn write_memory(&mut self, address: u16, data: u8) {
        self.memory.write(address, data);
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
            0x1000 => {
                let memory_address = instruction & 0xFFF;
                self.pc = memory_address;
            },
            0x2000 => {},
            0x3000 => {
                let byte = instruction & 0xFF;
                let vx = self.register[((instruction & 0x0F00) >> 8) as usize] as u16;

                if byte == vx {
                    self.pc += 2;
                }
            },
            0x4000 => {
                let byte = instruction & 0xFF;
                let vx = self.register[((instruction & 0x0F00) >> 8) as usize] as u16;

                if byte != vx {
                    self.pc += 2;
                }
            },
            0x5000 => {
                let vx = self.register[((instruction & 0x0F00) >> 8) as usize] as u16;
                let vy = self.register[((instruction & 0x00F0) >> 4) as usize] as u16;

                if vx == vy {
                    self.pc += 2;
                }
            },
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
                let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                let register_y_index = ((instruction & 0x00F0) >> 4) as usize;
                match instruction & 0xF {
                    0 => {
                        self.register[register_x_index] = self.register[register_y_index];
                    },
                    1 => {
                        self.register[register_x_index] = self.register[register_x_index] | self.register[register_y_index];
                    },
                    2 => {
                        self.register[register_x_index] = self.register[register_x_index] & self.register[register_y_index];
                    },
                    3 => {
                        self.register[register_x_index] = self.register[register_x_index] ^ self.register[register_y_index];
                    },
                    4 => {
                        let sum = (self.register[register_x_index] as u16) + (self.register[register_y_index] as u16);

                        if sum > 0xFF {
                            self.register[15] = 1;
                        } else {
                            self.register[15] = 0;
                        }

                        self.register[register_x_index] = (sum & 0xFF) as u8;
                    },
                    5 => {
                        if self.register[register_x_index] > self.register[register_y_index] {
                            self.register[15] = 1;
                        } else {
                            self.register[15] = 0;
                        }

                        self.register[register_x_index] = self.register[register_x_index] - self.register[register_y_index];
                    },
                    6 => {
                        self.register[15] = self.register[register_y_index] & 0x1;
                        self.register[register_x_index] = self.register[register_y_index] >> 1;
                    },
                    7 => {
                        if self.register[register_y_index] > self.register[register_x_index] {
                            self.register[15] = 1;
                        } else {
                            self.register[15] = 0;
                        }

                        self.register[register_x_index] = self.register[register_y_index] - self.register[register_x_index];
                    },
                    0xE => {
                        self.register[15] = (self.register[register_y_index] & 0x80) >> 7;
                        self.register[register_x_index] = self.register[register_y_index] << 1;
                    },
                    _ => { panic!("operand not available")}
                }
                
            },
            0x9000 => {
                let vx = self.register[((instruction & 0x0F00) >> 8) as usize] as u16;
                let vy = self.register[((instruction & 0x00F0) >> 4) as usize] as u16;

                if vx != vy {
                    self.pc += 2;
                }
            },
            0xA000 => {
                let memory_address = instruction & 0x0FFF;
                self.i = memory_address;
            },
            0xB000 => {
                let memory_address = instruction & 0xFFF;
                let v0 = self.register[0];

                self.pc = memory_address + v0 as u16;
            },
            0xC000 => {},
            0xD000 => {},
            0xE000 => {},
            0xF000 => {
                let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                match instruction & 0xFF {
                    0x7 => {
                        self.register[register_x_index] = self.delay_timer;
                    },
                    0xA => {},
                    0x15 => {
                        self.delay_timer = self.register[register_x_index];
                    },
                    0x18 => {
                        self.sound_timer = self.register[register_x_index];
                    },
                    0x1E => {
                        self.i += (self.register[register_x_index] as u16);
                    },
                    0x29 => {},
                    0x33 => {},
                    0x55 => {
                        for i in 0..register_x_index + 1 {
                            self.write_memory(self.i, self.register[i]);
                            self.i+= 1;
                        }
                    },
                    0x65 => {},
                    _ => { panic!("operand not available!")}
                }
            },
            _ => { panic!("opcode not available!")}
        }
    }
}