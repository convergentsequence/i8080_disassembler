use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::{env, process::exit};

mod opcodes;
mod classifier;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <rom file>", env!("CARGO_PKG_NAME"));
        exit(1);
    }

    let path = args.get(1).unwrap();
    fs::metadata(path).unwrap_or_else(|_e| {
        println!("File doesn't exist!");
        exit(2);
    });

    let mut rom_buf = BufReader::new(File::open(path).unwrap_or_else(|_e| {
        println!("Error reading rom!");
        exit(3);
    }));

    let mut rom: Vec<u8> = Vec::new();
    rom_buf.read_to_end(&mut rom).unwrap_or_else(|_e| {
        println!("Error reading rom!");
        exit(4);
    });

    let mut pc = 0usize;
    while pc < rom.len() {
        let (inst, consumed_bytes) = classifier::classify(&rom, &pc);

        println!("{:04x} {}", pc, inst);

        pc += consumed_bytes;
    }

}
