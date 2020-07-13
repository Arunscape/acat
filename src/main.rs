#![deny(clippy::all)]

use getopts::Options;
use rand::Rng;
use std::fs::File;
use std::usize::MAX;
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

fn read_file(files: &Vec<String>) -> () {
    let mut rng = rand::thread_rng();

    for file in files.iter().skip(1) {
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
fn read_file_plain(file: &String) -> () {
    let file = File::open(file);

    for line in BufReader::new(file.unwrap()).lines() {
        print!("{}", line.unwrap());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("A", "show-all", "equivalent to -vET");
    opts.optflag(
        "b",
        "number-nonblank",
        "number nonempty output lines, overrides -n",
    );
    opts.optflag("e", "", "equivalent to -vE");
    opts.optflag("E", "show-ends", "display $ at end of each line");
    opts.optflag("n", "number", "number all output lines");
    opts.optflag("s", "squeeze-blank", "suppress repeated empty output lines");
    opts.optflag("t", "", "equivalent to -vT");
    opts.optflag("T", "show-tabs", "display TAB characters as ^I");
    opts.optflag("u", "", "(ignored)");
    opts.optflag(
        "v",
        "show-nonprinting",
        "use ^ and M- notation, except for LFD and TAB",
    );
    opts.optflag("", "version", "output version information and exit");

    match args.len() {
        1 => read_stdin(),
        (2..=MAX) => read_file(&args),
        _ => eprintln!("todo, print help"),
    }
}
