const MEM_SIZE: usize = 4096;
const INTERPRETER_END: usize = 0x1ff;
const ROM_START: usize = 0x200;
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
    pc: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MemError {
    OutOfBound,
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = [0; MEM_SIZE];
        let pc: u16 = ROM_START.try_into().unwrap();
        // Setting the font from address 0x50in
        for (idx, val) in FONTS.iter().enumerate() {
            mem[idx + FONT_START] = *val;
        }
        Memory {
            mem,
            pc
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc+1;
        // TODO: Handle wrap around

    }

    pub fn inspect_mem(self, start_addr:usize, end_addr:usize) {
        for (offset, val) in self.mem[start_addr..end_addr].iter().enumerate() {
            println!("0x{:X}: 0x{:X}", start_addr+offset, val);
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

    pub fn fetch_instruction(&mut self) -> u16 {
        let valh: u16 = self.mem[self.pc as usize].into();
        self.increment_pc();
        let vall: u16 = self.mem[self.pc as usize].into();
        self.increment_pc();
        valh << 8 | vall
    }

    pub fn fetch_byte(&self, addr: u16) -> u8 {
        // println!("Fetching byte at addr= 0x{:4X}", addr);
        self.mem[addr as usize]
    }

    pub fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_memory_with_good_size() {
        let m: Memory = Memory::new();
        assert_eq!(m.mem.len(), MEM_SIZE);
    }

    #[test]
    fn test_start_ram_empty() {
        let test_data = vec!(1; MEM_SIZE-ROM_START);
        let mut m: Memory = Memory::new();
        let r = m.load_rom(&test_data);
        assert!(r.is_ok());
        for idx in 0..INTERPRETER_END {
            assert_eq!(m.mem[idx], 0, "Ram element {} should be 0 but is {}", idx, m.mem[idx]);
        }
    }

    #[test]
    fn test_load_data_too_big() {
        /* Make sure that if we load data bigger than the space reserved
         * for the ROM, we get an error out */
        let test_data = vec!(1; MEM_SIZE-ROM_START+1);
        let mut m: Memory = Memory::new();
        let r = m.load_rom(&test_data);
        assert!(r.is_err());
        let got = r.unwrap_err();
        assert_eq!(got, MemError::OutOfBound);
    }

    #[test]
    fn test_load_data_ok() {
        let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut m: Memory = Memory::new();
        let r = m.load_rom(&test_data);
        assert!(r.is_ok());
        for idx in 0..test_data.len() {
            assert_eq!(m.mem[idx+ROM_START], test_data[idx], "Ram element {} should be 0 but is {}", idx, m.mem[idx]);
        }


    }

}
