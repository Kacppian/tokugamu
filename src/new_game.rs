use std::io;
use std::io::Write;

pub fn start_game() -> () {
    println!("The game will start in ....");
    loop {
        read_input();
    }
}

fn read_input() -> () {
    print!("Outgoing<-");
    std::io::stdout().flush().expect("flush failed. -- Something went wrong --");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let trimmed = input.trim();
    if trimmed == "qc" {
        end_game();
    }
    else if trimmed == "q" {
        quit_app();
    }
    else {
        println!("Incoming->okay");
    }
}

fn quit_app() -> () {
    std::process::exit(0);
}

fn end_game() -> () {
    println!("Are you sure ?");
}
