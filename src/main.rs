#![deny(clippy::all)]

mod clap_app;
use clap_app::clap_app;

use rand::Rng;
use std::fs::File;
use std::{self, io, io::prelude::*, io::BufReader};

use random_color::{Color, Luminosity, RandomColor};

fn paint([r, g, b]: [u32; 3]) {
    let esc = char::from(0x1b);
    print!("{}[38;2;{};{};{}m", esc, r, g, b);
}

fn read_stdin(hue: Option<Color>, luminosity: Option<Luminosity>) {
    let mut colour = RandomColor::new();

    if let Some(h) = hue {
        colour.hue(h);
    }

    if let Some(l) = luminosity {
        colour.luminosity(l);
    }
    for line in io::stdin().lock().lines() {
        for c in line.unwrap().chars() {
            let colour = colour.to_rgb_array();
            paint(colour);
            print!("{}", c);
        }
    }
}

fn read_file(files: &[&str], hue: Option<Color>, luminosity: Option<Luminosity>) {
    let mut colour = RandomColor::new();
    if let Some(h) = hue {
        colour.hue(h);
    }

    if let Some(l) = luminosity {
        colour.luminosity(l);
    }
    for file in files.iter() {
        let file = File::open(file);
        for line in BufReader::new(file.unwrap()).lines() {
            for c in line.unwrap().chars() {
                let colour = colour.to_rgb_array();
                paint(colour);
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
    let hue = match matches.value_of("hue") {
        Some("mono") => Some(Color::Monochrome),
        Some("red") => Some(Color::Red),
        Some("orange") => Some(Color::Orange),
        Some("yellow") => Some(Color::Yellow),
        Some("green") => Some(Color::Green),
        Some("blue") => Some(Color::Blue),
        Some("purple") => Some(Color::Purple),
        Some("pink") => Some(Color::Pink),
        _ => None,
    };
    let luminosity = match matches.value_of("luminosity") {
        Some("random") => Some(Luminosity::Random),
        Some("bright") => Some(Luminosity::Bright),
        Some("light") => Some(Luminosity::Light),
        Some("dark") => Some(Luminosity::Dark),
        _ => None,
    };

    if matches.is_present("FILE") {
        let files: Vec<&str> = matches.values_of("FILE").unwrap().collect();
        read_file(&files, hue, luminosity);
    } else {
        read_stdin(hue, luminosity);
    }
}
