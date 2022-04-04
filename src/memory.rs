const MEM_SIZE: usize = 4096;
const INTERPRETER_END: usize = 0x1ff;
const ROM_START: usize = 0x200;

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
        let mem = [0; MEM_SIZE];
        let pc: u16 = 0;
        Memory {
            mem,
            pc
        }
    }

    pub fn increment_pc() {
       // TODO
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
