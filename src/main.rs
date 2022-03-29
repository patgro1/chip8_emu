use std::env;
use std::fs;


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
    println!("{:?}", content)
}
