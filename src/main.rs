use std::env;
use std::fs;

mod memory;

struct Registers {
    vx: [u8; 16],
    mem_reg: u16,  // NOTE: This is the I register but I wanted a better name here
}

struct Timers {
    sound: u8,
    delay: u8
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
    let m = memory::Memory::new();
    match m.load_rom(&content) {
        Err => prinln!("There was an error loading the ROM")
    }
    println!("{:?}", content)
}
