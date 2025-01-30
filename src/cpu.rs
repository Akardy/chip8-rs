use crate::memory::Memory;
use rand;

pub struct CPU {
    memory: Memory,
    vram: [[u8; 64]; 32],
    vram_flag: bool,
    register: [u8; 16],
    pc: u16,
    i: u16,
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
    sp: u8,
    keypad: [bool; 16],
    keypad_wait: bool,
    k_v: u8,
} 

impl CPU {
    pub fn new() -> Self {
        CPU {
            memory: Memory::new(),
            vram: [[0; 64]; 32],
            vram_flag: false,
            register: [0; 16],
            pc: 0x200,
            i: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            sp: 0,
            keypad: [false; 16],
            keypad_wait: false,
            k_v: 0
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
            0x0 => {
                match instruction & 0xFF {
                    0xE0 => {
                        self.vram = [[0; 64]; 32];
                    },
                    0xEE => {
                        if self.sp == 0 { panic!("Stack is empty") }
                        self.sp -= 1;
                        let address = self.stack[self.sp as usize];
                        self.pc = address;

                    },
                    _ => panic!("not available")
                }
            },
            0x1000 => {
                let memory_address = instruction & 0xFFF;
                self.pc = memory_address;
            },
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                let address = instruction & 0x0FFF;
                self.pc = address;
                self.sp += 1;
            },
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
                let vx_i = ((instruction & 0x0F00) >> 8) as usize;
                let vy_i = ((instruction & 0x00F0) >> 4) as usize;
                match instruction & 0xF {
                    0 => {
                        self.register[vx_i] = self.register[vy_i];
                    },
                    1 => {
                        self.register[vx_i] = self.register[vx_i] | self.register[vy_i];
                    },
                    2 => {
                        self.register[vx_i] = self.register[vx_i] & self.register[vy_i];
                    },
                    3 => {
                        self.register[vx_i] = self.register[vx_i] ^ self.register[vy_i];
                    },
                    4 => {
                        let sum = (self.register[vx_i] as u16) + (self.register[vy_i] as u16);

                        if sum > 0xFF {
                            self.register[15] = 1;
                        } else {
                            self.register[15] = 0;
                        }

                        self.register[vx_i] = (sum & 0xFF) as u8;
                    },
                    5 => {
                        if self.register[vx_i] > self.register[vy_i] {
                            self.register[15] = 1;
                        } else {
                            self.register[15] = 0;
                        }

                        self.register[vx_i] = self.register[vx_i] - self.register[vy_i];
                    },
                    6 => {
                        self.register[15] = self.register[vy_i] & 0x1;
                        self.register[vx_i] = self.register[vy_i] >> 1;
                    },
                    7 => {
                        if self.register[vy_i] > self.register[vx_i] {
                            self.register[15] = 1;
                        } else {
                            self.register[15] = 0;
                        }

                        self.register[vx_i] = self.register[vy_i] - self.register[vx_i];
                    },
                    0xE => {
                        self.register[15] = (self.register[vy_i] & 0x80) >> 7;
                        self.register[vx_i] = self.register[vy_i] << 1;
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
            0xC000 => {
                let vx_i = ((instruction & 0x0F00) >> 8) as usize;
                let random_value = rand::random::<u8>();
                let mask = (instruction & 0xFF) as u8;
                self.register[vx_i] = random_value & mask;
            },
            0xD000 => {
                let sprite_length = instruction & 0xF;

                let vx = self.register[((instruction & 0x0F00) >> 8) as usize] as usize % 64;
                let vy = self.register[((instruction & 0x00F0) >> 4) as usize] as usize % 32;
                self.register[15] = 0;

                for row in 0..sprite_length {
                    let byte = self.read_memory(self.i + row);
                    for col in 0..8 {
                        let pixel = (byte >> (7 - col)) & 0x1;

                        let current_pixel = self.vram[vx][vy];

                        if pixel == 1 {
                            if current_pixel == 1 {
                                self.register[15] = 1;
                            }
                            self.vram[vx + col][vy + row as usize] ^= 1;
                        }
                    }
                }
                self.vram_flag = true;


            },
            0xE000 => {
                let vx = self.register[((instruction & 0x0F00) >> 8) as usize];
                let is_pressed = self.keypad[vx as usize];
                match instruction & 0xFF {
                    0x9E => {
                        if is_pressed {self.pc += 2;}
                    },
                    0xA1 => {
                        if !is_pressed {self.pc += 2;}
                    },
                    _ => panic!("not available!")
                }
            },
            0xF000 => {
                let vx_i = ((instruction & 0x0F00) >> 8) as usize;
                match instruction & 0xFF {
                    0x7 => {
                        self.register[vx_i] = self.delay_timer;
                    },
                    0xA => {}, // WIP
                    0x15 => {
                        self.delay_timer = self.register[vx_i];
                    },
                    0x18 => {
                        self.sound_timer = self.register[vx_i];
                    },
                    0x1E => {
                        self.i += (self.register[vx_i] as u16);
                    },
                    0x29 => {
                        self.i = (self.register[vx_i] as u16) * 5;
                    },
                    0x33 => {
                        let vx = self.register[vx_i];
                        self.write_memory(self.i, vx / 100);
                        self.write_memory(self.i + 1, (vx % 100) / 10 );
                        self.write_memory(self.i + 2, vx % 10);
                    },
                    0x55 => {
                        for i in 0..vx_i + 1 {
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