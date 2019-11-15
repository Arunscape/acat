use std::io;
use std::io::Read;
use rand::Rng;

fn paint(r:u8, g:u8, b:u8) {
    let esc = char::from(0x1b);
    print!("{}[38;2;{};{};{}m", esc, r, g, b);
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut rng = rand::thread_rng();
    for c in buffer.chars() {

        paint(rng.gen(), rng.gen(), rng.gen());
        print!("{}" ,c);        
    }

    println!();


}
