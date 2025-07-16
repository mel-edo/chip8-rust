use std::fs::read;

impl Cpu {
    pub fn new() -> Self {
       Self { 
            opcode: 0, 
            memory: [0; 4096], 
            v: [0; 16], 
            i: 0, 
            pc: 0x200, 
            gfx: [0; 64*32], 
            sound_timer: 0, 
            delay_timer: 0, 
            stack: [0; 16], 
            sp: 0, 
            keypad: [0; 16]
        } 
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

        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1]) as u16;

        println!("Opcode: {:#06x}, pc: {}", self.opcode, self.pc);

        // increment program counter by 2
        self.pc += 2;

        // decode the opcode
        // execute the instruction
        
        // update timers if needed(decrement both timers if they are > 0)
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        } else if self.delay_timer > 0 {
            self.delay_timer -= 1;
        };

        Ok(())
    }
}

pub struct Cpu {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: usize,
    gfx: [u8; 64*32],
    sound_timer: u8,
    delay_timer: u8,
    stack: [u16; 16],
    sp: u8,
    keypad: [u8; 16],
}