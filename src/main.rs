use std::io;
use rand::Rng;
use std::io::prelude::*;

fn paint(r:u8, g:u8, b:u8) {
    let esc = char::from(0x1b);
    print!("{}[38;2;{};{};{}m", esc, r, g, b);
}

fn main() {
    let mut rng = rand::thread_rng();

        for line in io::stdin().lock().lines() {

            for c in line.unwrap().chars() {

            paint(rng.gen(), rng.gen(), rng.gen());
            print!("{}" ,c);        
        }
            println!();
    }

    println!();


}
