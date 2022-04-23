use std::{env, fs};
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::Sdl;

pub const BASE_WIDTH: u32 = 64;
pub const BASE_HEIGHT: u32 = 32;
pub const BUFFER_SIZE: usize = BASE_WIDTH as usize * BASE_HEIGHT as usize;

mod chip8;
mod memory;
mod screen;


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
    let mut screen: screen::Screen = screen::Screen::new(&sdl_context, 20);
    let mut vram: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut cpu = chip8::Chip8::new();
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
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'main_loop
                    },
                _ => {}
            }
        }
        cpu.tick(&mut vram);
        screen.draw(&vram);
    }
}
