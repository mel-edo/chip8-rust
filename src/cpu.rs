use std::{fs::read};
use fastrand;

impl Cpu {
    pub fn new() -> Self {
       let mut cpu = Self { 
            opcode: 0, 
            memory: [0; 4096], 
            v: [0; 16], 
            i: 0, 
            pc: 0x200, 
            gfx: [0; 64*32],
            draw_flag: false,
            sound_timer: 0, 
            delay_timer: 0, 
            stack: [0; 16], 
            sp: 0, 
            keypad: [0; 16]
        };
        for (i, &byte) in FONTSET.iter().enumerate() {
            cpu.memory[0x050 + i] = byte;
        };

        cpu        
    }
    
    pub fn load_rom(&mut self, path: &str) -> Result<(), std::io::Error> {
        let data: Vec<u8>  = read(path)?;
        let mut addr: usize = 0x200;

        if data.len() > (4096 - 512) {
            panic!("ROM is too large to fit into the memory!");
        }

        for i in data {
            self.memory[addr] = i;
            addr += 1;
        }
        Ok(())
    }

    pub fn cycle(&mut self) -> Result<(), std::io::Error> {
        // Fetch the opcode

        // suppose memory[pc] = 0xA2 -> binary: 10100010 (this is u8)
        // memory[pc+1] = 0xF0 -> binary: 11110000 (also u8)
        
        // 0xA2 gets casted as u16 -> 00000000 10100010
        // Thus, we will shift it by 8

        // Now we have 10100010 00000000
        // and the second 00000000 11110000
        // we can simply xor them to get the u16 opcode
        
        // If pc > 4096, it will go out of memory bounds
        if self.pc > 4095 {  // As 4095 and 4096 will be 2 byte opcode
            panic!("pc went out of memory bounds");
        };

        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1]) as u16;

        println!("Opcode: {:#06x}, pc: {}", self.opcode, self.pc);


        // decode the opcode and execute the instruction
        match self.opcode & 0xF000 {
            0x0000 => self.op_00nn(),
            0x1000 => self.op_1nnn(),
            0x2000 => self.op_2nnn(),
            0x3000 => self.op_3xkk(),
            0x4000 => self.op_4xkk(),
            0x5000 => self.op_5xy0(),
            0x6000 => self.op_6xkk(),
            0x7000 => self.op_7xkk(),
            0x8000 => self.op_8xyn(),
            0x9000 => self.op_9xy0(),
            0xA000 => self.op_annn(),
            0xB000 => self.op_bnnn(),
            0xC000 => self.op_cxkk(),
            0xD000 => self.op_dxyn(),
            0xE000 => self.op_exnn(),
            0xF000 => self.op_fxnn(),
            _ => println!("opcode not covered yet: {:#06x}", self.opcode),
        }
        
        // update timers if needed(decrement both timers if they are > 0)
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        } else if self.delay_timer > 0 {
            self.delay_timer -= 1;
        };

        Ok(())
    }

    fn op_00nn(&mut self) {
        // 00E0 - clear the display
        // 00EE - return from subroutine
        match self.opcode & 0x000F {
            0x0000 => {
                self.gfx.fill(0);
                self.draw_flag = true;
                self.pc += 2
            },
            0x000E => {
                self.sp -= 1;
                self.pc = self.stack[self.sp] as usize;
            },
            _ => println!("opcode not covered yet: {:#06x}", self.opcode),
        }
    }

    fn op_1nnn(&mut self) {
        // jumps pc to nnn
        let nnn: u16 = self.opcode & 0x0FFF;
        self.pc = nnn as usize;
    }

    fn op_2nnn(&mut self) {
        // jump to subroutine at nnn
        let nnn = self.opcode & 0x0FFF;
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = nnn as usize;
    }

    fn op_3xkk(&mut self) {
        // skip next instruction if Vx == kk
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk: u8 = (self.opcode & 0x00FF) as u8;
        if self.v[x] == kk {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn op_4xkk(&mut self) {
        // skip next instruction if Vx != kk
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk: u8 = (self.opcode & 0x00FF) as u8;
        if self.v[x] != kk {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn op_5xy0(&mut self) {
        // skip next instruction if Vx == Vy
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
        if self.v[x] == self.v[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn op_6xkk(&mut self) {
        // load kk into V[x]
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk: u8 = (self.opcode & 0x00FF) as u8;
        self.v[x] = kk;
        self.pc += 2;
    }

    fn op_7xkk(&mut self) {
        // add kk to existing value of V[x]
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk: u8 = (self.opcode & 0x00FF) as u8;
        self.v[x] = self.v[x].wrapping_add(kk);
        self.pc += 2;
    }

    fn op_8xyn(&mut self) {
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
        match self.opcode & 0x000F {
            0x0000 => self.v[x] = self.v[y],
            0x0001 => self.v[x] |= self.v[y],
            0x0002 => self.v[x] &= self.v[y],
            0x0003 => self.v[x] ^= self.v[y],
            // 8xy4 -> Vx = Vx + Vy. if result greater than 8 bits, Vf = 1
            0x0004 => {
                let (result, did_overflow) = self.v[x].overflowing_add(self.v[y]);
                if did_overflow {
                    self.v[0xF] = 1;  // since 0xF is the 16th register
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = result;
            },
            0x0005 => {
                let (result, did_underflow) = self.v[x].overflowing_sub(self.v[y]);
                if !did_underflow {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = result;
            },
            0x0006 => {
                if self.v[x] & 1 == 1 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] /= 2;
            },
            0x0007 => {
                let (result, did_underflow) = self.v[y].overflowing_sub(self.v[x]);
                if !did_underflow {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = result;
            },
            0x000E => {
                if self.v[x] & 0b10000000 != 0 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = self.v[x] << 1;
            }
            _ => println!("opcode not covered yet: {:#06x}", self.opcode),
        }
        self.pc += 2;
    }

    fn op_9xy0(&mut self) {
        // skip next instruction if Vx != Vy
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
        if self.v[x] != self.v[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn op_annn(&mut self) {
        // set i to nnn
        let nnn: u16 = self.opcode & 0x0FFF;
        self.i = nnn;
        self.pc += 2;
    }

    fn op_bnnn(&mut self) {
        // jump to nnn + v0
        let nnn: u16 = self.opcode & 0x0FFF;
        self.pc = (nnn + (self.v[0] as u16)) as usize;
    }

    fn op_cxkk(&mut self) {
        // Vx = random byte & kk
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk: u8 = (self.opcode & 0x00FF) as u8;
        self.v[x] = fastrand::u8(..) & kk;
        self.pc += 2;
    }

    fn op_dxyn(&mut self) {
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
        let n: usize = (self.opcode & 0x000F) as usize;

        self.v[0xF] = 0;  // This will track if any pixels were flipped from 1 to 0 during drawing

        for row in 0..n {
            let sprite_byte = self.memory[self.i as usize + row];
            for bit in 0..8 {
                let pixel_val = (sprite_byte >> (7 - bit)) & 1;  // we're extracting the current pixel here
                let x_pos = (self.v[x] as usize + bit) % 64;
                let y_pos = (self.v[y] as usize + row) % 32;
                let index = y_pos * 64 + x_pos;

                if self.gfx[index] == 1 && pixel_val == 1 {
                    self.v[0xF] = 1;
                }
                self.gfx[index] ^= pixel_val;
            }
        }
        self.draw_flag = true;
        self.pc += 2;
    }

    fn op_exnn(&mut self) {
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        match self.opcode & 0x000F {
            0x000E => {
                // pc += 2 if key with Vx value pressed
                if self.keypad[self.v[x] as usize] == 1 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x0001 => {
                // pc += 2 if key with Vx not pressed
                if self.keypad[self.v[x] as usize] != 1 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            _ => println!("opcode not covered yet {:#06x}", self.opcode),
        }
    }

    fn op_fxnn(&mut self) {
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        match self.opcode & 0x000F {
            0x0007 => {
                self.v[x] = self.delay_timer;
                self.pc += 2;
            },
            0x000A => {
                if let Some(pressed_key) = self.keypad.iter().position(|&k| k == 1) {
                    self.v[x] = pressed_key as u8;
                    self.pc += 2;
                } else {
                    return;
                }
            },
            0x0005 => {
                match self.opcode & 0x00F0 {
                    0x0010 => self.delay_timer = self.v[x],
                    0x0050 => {
                        if self.i as usize + x >= self.memory.len() {
                            panic!("Memory write out of bounds in 0xFX55");
                        };

                        for register in 0..=x {
                            self.memory[self.i as usize + register] = self.v[register];
                        }
                    },
                    0x0060 => {
                        if self.i as usize + x >= self.memory.len() {
                            panic!("Memory read out of bounds in 0xFX65");
                        };

                        for register in 0..=x {
                            self.v[register] = self.memory[self.i as usize + register];
                        }
                    },
                    _ => println!("opcode not covered yet {:#06x}", self.opcode),
                }
                self.pc += 2;
            },
            0x0008 => {
                self.sound_timer = self.v[x];
                self.pc += 2;
            },
            0x000E => {
                self.i += self.v[x] as u16;
                self.pc += 2;
            },
            0x0009 => {
                self.i = 0x050 + (self.v[x] as u16) * 5;
                self.pc += 2;
            },
            0x0003 => {
                let val = self.v[x];
                self.memory[self.i as usize] = val / 100;
                self.memory[self.i as usize + 1] = (val % 100) / 10;
                self.memory[self.i as usize + 2] = val % 10;
                self.pc += 2;
            },
            _ => println!("opcode not covered yet {:#06x}", self.opcode),
        }
    }
}

const FONTSET: [u8; 80] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub struct Cpu {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: usize,
    pub gfx: [u8; 64*32],
    pub draw_flag: bool,
    sound_timer: u8,
    delay_timer: u8,
    stack: [u16; 16],
    sp: usize,
    pub keypad: [u8; 16],
}