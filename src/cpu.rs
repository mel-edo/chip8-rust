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
}

pub struct Cpu {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    gfx: [u8; 64*32],
    sound_timer: u8,
    delay_timer: u8,
    stack: [u16; 16],
    sp: u8,
    keypad: [u8; 16],
}