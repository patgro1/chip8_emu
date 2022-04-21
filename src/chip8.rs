use num_enum::TryFromPrimitive;

use crate::memory::Memory;
use crate::memory::ROM_START;
use crate::memory::MemError;
use crate::traits::{ScreenDisplay, BUFFER_SIZE};


#[derive(TryFromPrimitive)]
#[derive(Debug)]
#[repr(u16)]
enum OpCode {
    ClearScreen = 0x0,
    Jump = 0x1,
    SetVx = 0x6,
    AddVx = 0x7,
    SetI = 0xA,
    Draw = 0xD
}

pub struct Chip8<T: ScreenDisplay> {
    vx: [u8; 16],
    mem_reg: u16,  // NOTE: This is the I register
    sound_timer: u8,
    delay_timer: u8,
    pc: u16,
    mem: Memory,
    display: T,
    display_buffer: [u8; BUFFER_SIZE]
}

impl<T: ScreenDisplay> Chip8<T> {
    pub fn new(display: T) -> Self {
        Chip8 {
            vx: [0; 16],
            mem_reg: 0,
            sound_timer: 0,
            delay_timer: 0,
            pc: 0,
            display: display,
            mem: Memory::new(),
            display_buffer: [0; BUFFER_SIZE]}

    }

    pub fn reset(&mut self) {
        self.vx = [0; 16];
        self.mem_reg = 0;
        self.sound_timer = 0;
        self.delay_timer = 0;
        self.pc = ROM_START as u16;
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) -> Result<(), MemError> {
        self.mem.load_rom(data)
    }

    pub fn run(&mut self, instruction_cnt: u32) {
        let mut current_instr_cnt = 0;
        while current_instr_cnt < instruction_cnt {
            let instruction: u16 = self.fetch_next_instruction();
            self.pc += 2;
            println!("instruction: 0x{:4X}", instruction);
            let op_code: OpCode = OpCode::try_from(instruction >> 12).unwrap();
            let nibbles: [u16; 3] = [(instruction&0x0F00) >> 8,
                                    (instruction&0x00F0) >> 4,
                                    instruction&0x000F];
            println!("Instruction: {:?} with nibbles {:?}", op_code, nibbles);
            match op_code {
                OpCode::ClearScreen=> {
                    self.display_buffer = [0; BUFFER_SIZE];
                    self.display.draw(self.display_buffer);
                },
                OpCode::Jump => self.pc = instruction & 0x0FFF,
                OpCode::SetVx => {
                    self.vx[nibbles[0] as usize] = (nibbles[1] as u8) <<4 | nibbles[2] as u8;
                    println!("new vx = {:?}", self.vx);
                },
                OpCode::AddVx => {
                    self.vx[nibbles[0] as usize] += (nibbles[1] as u8) <<4 | nibbles[2] as u8;
                    println!("new vx = {:?}", self.vx);
                    // TODO: manage flags
                },
                OpCode::SetI => self.mem_reg = instruction & 0x0FFF,
                OpCode::Draw => {
                    println!("TODO: recalculate the buffer");
                    self.calculate_display_buffer(nibbles[0] as u8, nibbles[1] as u8, nibbles[2] as u8);
                    self.display.draw(self.display_buffer);
                }
            }
            current_instr_cnt += 1;
        }
    }

    fn fetch_next_instruction(&mut self) -> u16{
        let valh: u16 = (self.mem.fetch_byte(self.pc)).into();
        let vall: u16 = (self.mem.fetch_byte(self.pc+1)).into();
        ((valh  << 8)| (vall)).into()

    }
    fn calculate_display_buffer(&mut self, vx: u8, vy: u8, n:u8) {
        let x: u16 = (self.vx[vx as usize]) as u16 % 64;
        let y: u16 = (self.vx[vy as usize])as u16 % 32;
        // let mut buffer = old_buffer.clone();
        // println!("n = {}", n);
        // TODO: set vf to 0 here
        // print_screen_buffer(&buffer);
        for y_off in 0_u16..n as u16 {
            let mem_location = self.mem_reg + y_off as u16;
            let byte: u8 = self.mem.fetch_byte(mem_location);
            // println!("{:X}, {:X}", mem_location, byte);
            for x_off in 0..8_u16{
                if x + x_off > 63 {break};
                let buffer_index: usize = ((y + y_off) * 64 + x + x_off).into();
                // println!("x={}, y={}, x_off={}, y_off={}, buffer_index={}", x, y, x_off, y_off, buffer_index);
                // The MSB is in fact the first bit we need to check
                let bit_val = (byte >> 7-x_off) & 1;
                // println!("byte={:02X}, bit_val={}", byte, bit_val);
                // println!("value before xor: {}", buffer[buffer_index as usize]);
                self.display_buffer[buffer_index as usize] = self.display_buffer[buffer_index as usize] ^ bit_val;
                // println!("value after xor: {}", buffer[buffer_index as usize]);
            }
        }
    }

    // fn calculate_display_buffer(&mut self,
}
