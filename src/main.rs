use std::{env, fs};

use std::{thread, time};
use sdl2::{Sdl, EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
    let sdl_context:Sdl = sdl2::init().unwrap();
    let screen: screen::Screen = screen::Screen::new(&sdl_context, 20);
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

    // Setting the main loop
    let mut event_pump: EventPump = sdl_context.event_pump().unwrap();

    // Main loop
    cpu.reset();

    'main_loop: loop {
        cpu.tick();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'main_loop
                    },
                _ => {}
            }
        }
    }
}
