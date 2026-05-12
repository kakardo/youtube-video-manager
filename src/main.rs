use std::io;
use std::io::Write;

fn main() {
    let mut choice = String::new();

    while choice.trim() != "3" {
        println!("\nVIDEO MANAGER MENU");
        println!("(1) Show video");
        println!("(2) Mark video as watched");
        println!("(3) Exit");

        choice.clear(); // remove previous option
        print!("Option: ");
        
        io::stdout()
            .flush()
            .expect("Failed to flush stdout");
    
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        //println!("You wrote: {}", choice.trim());d
        
        // String match
        match choice.trim() {
            "1" => println!("Show video"),
            "2" => println!("Mark video as watched"),
            "3" => println!("Exit"),
            _ => println!("Invalid option!"),
        }
    }

}
