use std::io;
use std::io::Write;




fn main() {
    let videos = [
        "https://www.youtube.com/watch?v=bLHL75H_VEM",
        "https://www.youtube.com/watch?v=u3CKgkyc7Qo",
        "https://www.youtube.com/watch?v=dxIPcbmo1_U",
        "https://www.youtube.com/watch?v=s-mOy8VUEBk",
        "https://www.youtube.com/watch?v=e1ozCWyUlCg",
        "https://www.youtube.com/watch?v=hcxwTgEC7IM",
        ];
        
    let mut watched = [
        false, false, false, false, false, false
    ];
        
    let mut choice = String::new();

    while choice.trim() != "3" {
        print_menu();
        get_user_choice(&mut choice);
    
        // String match
        match choice.trim() {
            "1" => show_video_list(&videos),
            "2" => println!("Mark video as watched"),
            "3" => println!("Exit"),
            _ => println!("Invalid option!"),
        }
    }
    
}

// Prints the program menu
fn print_menu() {
    println!("\nVIDEO MANAGER MENU");
    println!("(1) Show video");
    println!("(2) Mark video as watched");
    println!("(3) Exit");
}

fn get_user_choice(choice: &mut String) {
    choice.clear(); // remove previous option
    print!("Option: ");
    
    io::stdout()
        .flush()
        .expect("Failed to flush stdout");
        
    io::stdin()
    .read_line(choice)
    .expect("Failed to read line");
    //println!("You wrote: {}", choice.trim());
}

// Shows video URL:s contained in array
fn show_video_list(videos: &[&str]) {
    println!("> Show video");
    
    for (index, video) in videos.iter().enumerate() {
        println!("{} {}", index, video);
    }
}