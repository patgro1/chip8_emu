const MEM_SIZE: usize = 4096;
pub const ROM_START: usize = 0x200;
const FONT_START: usize = 0x50;
const FONT_ARR_SIZE: usize = 5*16;
//const NUMBERS: &'static [i32] = &[1, 2, 3, 4, 5];
const FONTS: [u8; FONT_ARR_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Memory {
    mem: [u8; MEM_SIZE as usize],
    /* NOTE: Even though the PC should be part of the cpu, it is tightly coupled to the mem so I
     decided to put it in the same structure.*/
}

#[derive(Debug, PartialEq, Eq)]
pub enum MemError {
    OutOfBound,
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = [0; MEM_SIZE];
        // Setting the font from address 0x50in
        for (idx, val) in FONTS.iter().enumerate() {
            mem[idx + FONT_START] = *val;
        }
        Memory {
            mem,
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) -> Result<(), MemError> {
        if data.len() > MEM_SIZE-ROM_START {
            return Err(MemError::OutOfBound)
        }
        // The first 512 bytes are used for the interpreter. When loading a rom, we should leave
        // them empty.
        for (idx, current_data) in data.iter().enumerate() {
            self.mem[0x200 + idx] = *current_data;
        }
        return Ok(())
    }

    pub fn fetch_byte(&self, addr: u16) -> u8 {
        // println!("Fetching byte at addr= 0x{:4X}", addr);
        self.mem[addr as usize]
    }

}
