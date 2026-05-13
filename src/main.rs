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
        get_menu_choice(&mut choice);
    
        // String match
        match choice.trim() {
            "1" => list_unwatched_videos(&videos, &watched),
            "2" => watched_video_toggle(&videos, &mut watched),
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

// Gets users choice
fn get_menu_choice(choice: &mut String) {
    print!("Option: ");
    get_input(choice);
    //println!("You wrote: {}", choice.trim());
}

fn get_input(input: &mut String) {
    input.clear(); // remove previous option
    io::stdout()
        .flush()
        .expect("Failed to flush stdout");
        
    io::stdin()
    .read_line(input)
    .expect("Failed to read line");
}

// Shows video URL:s contained in array
fn list_unwatched_videos(videos: &[&str], watched: &[bool]) {
    println!("> Show video");
    
    for (index, video) in videos.iter().enumerate() {
        if !watched[index] {
            println!("{} {}", index, video);
        }
        
    }
}

fn watched_video_toggle(videos: &[&str], watched: &mut [bool]) {
    println!("> Mark video as watched");

    let mut video_index = String::new();
    print!("Give index of video: ");
    get_input(&mut video_index);

    // Checks if its fit to be usize, or not
    // Needs match to return a usize
    let i: usize = match video_index.trim().parse::<usize>() {
        Ok(is_valid) => {
            println!("Valid <usize> received: {}", is_valid);
            is_valid
        }
        Err(_) => {
            println!("Not a valid index: {}", video_index);
            return;
        }
    };

    if i >= videos.len() {
        println!("No index match: {}", i);
        return;
    }

    println!("{:?} {:?}", videos[i], watched[i]);
    watched[i] = !watched[i];
    println!("Toggling watched status: {}", watched[i]);

    if watched[i] {
        println!("Marked as watched: {}", videos[i]);
    } else {
        println!("Marked as unwatched: {}", videos[i]);
    }

}