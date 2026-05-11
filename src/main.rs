use std::io;
use std::io::Write;

fn main() {
    println!("Hej hej världen!");

    let mut choice = String::new();
    print!("Write anything: ");


    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    
    println!("You wrote: {}", choice.trim());
}
