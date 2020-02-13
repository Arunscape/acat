use rand::Rng;
use std::fs::File;
use std::{self, env, io, io::prelude::*, io::BufReader};

fn paint(r: u8, g: u8, b: u8) {
    let esc = char::from(0x1b);
    print!("{}[38;2;{};{};{}m", esc, r, g, b);
}

fn read_stdin() -> () {
    let mut rng = rand::thread_rng();
    for line in io::stdin().lock().lines() {
        for c in line.unwrap().chars() {
            paint(rng.gen(), rng.gen(), rng.gen());
            print!("{}", c);
        }
    }
}

fn read_file(file: &String) -> () {
    let mut rng = rand::thread_rng();
    let file = File::open(file);

    for line in BufReader::new(file.unwrap()).lines() {
        for c in line.unwrap().chars() {
            paint(rng.gen(), rng.gen(), rng.gen());
            print!("{}", c);
        }
        println!();
    }
}

#[allow(dead_code)]
fn read_file_plain(file: &String) -> () {
    let file = File::open(file);

    for line in BufReader::new(file.unwrap()).lines() {
        print!("{}", line.unwrap());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => read_stdin(),
        2 => read_file(&args[1]),
        _ => eprintln!("todo, print help"),
    }
}
