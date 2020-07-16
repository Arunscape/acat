#![deny(clippy::all)]

mod clap_app;
use clap_app::clap_app;

use rand::Rng;
use std::fs::File;
use std::{self, io, io::prelude::*, io::BufReader};

fn paint(r: u8, g: u8, b: u8) {
    let esc = char::from(0x1b);
    print!("{}[38;2;{};{};{}m", esc, r, g, b);
}

fn read_stdin() {
    let mut rng = rand::thread_rng();
    for line in io::stdin().lock().lines() {
        for c in line.unwrap().chars() {
            paint(rng.gen(), rng.gen(), rng.gen());
            print!("{}", c);
        }
    }
}

fn read_file(files: &Vec<&str>) {
    let mut rng = rand::thread_rng();

    for file in files.iter() {
        let file = File::open(file);

        for line in BufReader::new(file.unwrap()).lines() {
            for c in line.unwrap().chars() {
                paint(rng.gen(), rng.gen(), rng.gen());
                print!("{}", c);
            }
            println!();
        }
    }
}

#[allow(dead_code)]
fn read_file_plain(file: &str) {
    let file = File::open(file);

    for line in BufReader::new(file.unwrap()).lines() {
        print!("{}", line.unwrap());
    }
}

fn main() {
    let matches = clap_app().get_matches();

    if matches.is_present("FILE") {
        let files: Vec<&str> = matches.values_of("FILE").unwrap().collect();
        read_file(&files);
    } else {
        read_stdin();
    }
}
