use std::{env, fs};

use std::{thread, time};

mod chip8;
mod memory;
mod screen;
mod traits;


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
    let screen: screen::Screen = screen::Screen::new(20);
    let mut cpu = chip8::Chip8::new(screen);
    match cpu.load_rom(&content) {
        Err(v) => match v {
            memory::MemError::OutOfBound => {
                println!("The rom file is too big for the amount of available memory");
                std::process::exit(-1);
            },
        }
        _ => println!("{:X?}", content)
    }
    // For now, we will execute a fix amount of instructions
    let max_instructions = 25;

    // Main loop
    cpu.reset();
    cpu.run(max_instructions);

    // let mut screen = screen::Screen::new(20);
    // screen.reset();
    // screen.draw_pixel(0,0);
    // screen.draw_pixel(2,0);
    // screen.show();
    let five_seconds = time::Duration::from_secs(10);
    thread::sleep(five_seconds);
}
