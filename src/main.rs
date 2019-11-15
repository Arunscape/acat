use std::io;
use std::io::Read;

fn main() -> io::Result<()> {

    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    for line in buffer {
        println!("{}", line);
    }
    println!("Hello, world!");

    Ok(())
}
