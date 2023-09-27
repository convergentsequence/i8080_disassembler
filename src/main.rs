use std::{env, process::exit};
use std::fs;

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




}
