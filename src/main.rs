use std::{env, fs};

use std::{thread, time};

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

mod memory;
mod screen;

#[derive(Debug)]
struct Registers {
    vx: [u8; 16],
    mem_reg: u16,  // NOTE: This is the I register but I wanted a better name here
}

struct Timers {
    sound: u8,
    delay: u8
}
    // 00E0 (clear screen)
    // 1NNN (jump)
    // 6XNN (set register VX)
    // 7XNN (add value to register VX)
    // ANNN (set index register I)
    // DXYN (display/draw)
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

struct Config {
    rom_filename: String
}

fn parse_cfg(args: Vec<String>) -> Config {
    let rom_filename = args[1].clone();
    Config{rom_filename}
    // TODO: Error handling
}

fn main() {
    // TODO: Move the config in a seperate module before it grows too much
    let args: Vec<String> = env::args().collect();
    let config = parse_cfg(args);

    // TODO: Should this be placed in a different part of the code? Memory loading for example?
    let content = match fs::read(config.rom_filename) {
        Err(error) => {
            println!("there was an error reading the file: {}", error);
            std::process::exit(-1);
        },
        Ok(val) => val
    };
    let mut m = memory::Memory::new();
    match m.load_rom(&content) {
        Err(v) => match v {
            memory::MemError::OutOfBound => {
                println!("The rom file is too big for the amount of available memory");
                std::process::exit(-1);
            },
        }
        _ => println!("{:X?}", content)
    }
    let mut screen: screen::Screen = screen::Screen::new(20);
    let mut buffer: [u8; screen::BUFFER_SIZE] = [0; screen::BUFFER_SIZE];
    // For now, we will execute a fix amount of instructions
    let max_instructions = 25;
    let mut current_instr_cnt = 0;
    let mut reg: Registers = Registers{vx: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], mem_reg: 0};

    while current_instr_cnt < max_instructions {
        // Fetch instruction
        let instruction: u16= m.fetch_instruction();
        println!("instruction: 0x{:4X}", instruction);
        let op_code: OpCode = OpCode::try_from(instruction >> 12).unwrap();
        let nibbles: [u16; 3] = [(instruction&0x0F00) >> 8,
                             (instruction&0x00F0) >> 4,
                             instruction&0x000F];
        println!("Instruction: {:?} with nibbles {:?}", op_code, nibbles);
        // Decode instruction
        match op_code {
            OpCode::ClearScreen=> {
                buffer = [0; screen::BUFFER_SIZE];
                screen.draw(&buffer);
            },
            OpCode::Jump => m.jump(instruction & 0x0FFF),
            OpCode::SetVx => {
                reg.vx[nibbles[0] as usize] = (nibbles[1] as u8) <<4 | nibbles[2] as u8;
                println!("new reg = {:?}", reg);
            },
            OpCode::AddVx => {
                reg.vx[nibbles[0] as usize] += (nibbles[1] as u8) <<4 | nibbles[2] as u8;
                println!("new reg = {:?}", reg);
                // TODO: manage flags
            },
            OpCode::SetI => reg.mem_reg = instruction & 0x0FFF,
            OpCode::Draw => {
                buffer = calculate_display_buffer(&buffer, nibbles[0] as u8, nibbles[1] as u8, nibbles[2] as u8, &reg, &m);
                screen.draw(&buffer);
            }
        }

        current_instr_cnt = current_instr_cnt+1;
    }
    // Main loop



    // let mut screen = screen::Screen::new(20);
    // screen.reset();
    // screen.draw_pixel(0,0);
    // screen.draw_pixel(2,0);
    // screen.show();
    let five_seconds = time::Duration::from_secs(10);
    thread::sleep(five_seconds);
}

// FIXME: This starts to get ugly... we should totally have a class implement the cpu state so we
// can have function without that kind of ugly signature
fn calculate_display_buffer(&(mut old_buffer): &[u8; screen::BUFFER_SIZE],
                            vx: u8,
                            vy:u8,
                            n: u8,
                            reg: &Registers,
                            mem: &memory::Memory) -> [u8; screen::BUFFER_SIZE]{
    let x: u16 = (reg.vx[vx as usize]) as u16 % 64;
    let y: u16 = (reg.vx[vy as usize])as u16 % 32;
    let mut buffer = old_buffer.clone();
    println!("n = {}", n);
    // TODO: set vf to 0 here
    // print_screen_buffer(&buffer);
    for y_off in 0_u16..n as u16 {
        let mem_location = reg.mem_reg + y_off as u16;
        let byte: u8 = mem.fetch_byte(mem_location);
        // println!("{:X}, {:X}", mem_location, byte);
        for x_off in 0..8_u16{
            if x + x_off > 63 {break};
            let buffer_index: usize = ((y + y_off) * 64 + x + x_off).into();
            // println!("x={}, y={}, x_off={}, y_off={}, buffer_index={}", x, y, x_off, y_off, buffer_index);
            // The MSB is in fact the first bit we need to check
            let bit_val = (byte >> 7-x_off) & 1;
            // println!("byte={:02X}, bit_val={}", byte, bit_val);
            // println!("value before xor: {}", buffer[buffer_index as usize]);
            buffer[buffer_index as usize] = buffer[buffer_index as usize] ^ bit_val;
            // println!("value after xor: {}", buffer[buffer_index as usize]);
        }
    }
    print_screen_buffer(&buffer);
    buffer
}

fn print_screen_buffer(&buffer: &[u8; screen::BUFFER_SIZE]) {
    for y in 0..32usize {
        for x in 0..64usize {
            let buffer_index = y * 64 + x;
            print!("{}", buffer[buffer_index])
        }
        print!("\n")
    }
}
