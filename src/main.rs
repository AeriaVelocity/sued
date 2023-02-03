// main.rs - sued, the text editor of all time, short for Shut Up Editor

use std::io;
use std::fs;
use rand::Rng;

fn startup_message() {
    let messages: Vec<&str> = vec!["the editor of all time", "shut up and edit", "the nonstandard text editor", "sued as in editor, not as in law"];
    let message: &str = messages[rand::thread_rng().gen_range(0..messages.len())];
    println!("sued - {message}\ntype ~ for commands");
}

fn display_help() {
    println!("~save, ~open, ~show, ~exit");
}

fn save() {
    println!("sorry, can't save yet");
}

fn show() {
    println!("sorry, no file buffer yet");
}

fn open(command_args: Vec<&str>) -> &str {
    if command_args.len() <= 1 {
        println!("open what?");
        return "";
    }
    else {
        return command_args[1];
    }
}

fn main() {
    startup_message();
    let mut command: String = String::new();
    while !command.eq("~exit") {
        command.clear();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command.");
        let len: usize = command.trim_end_matches(&['\r', '\n'][..]).len();
        command.truncate(len);
        let command_args = command.split(" ").collect::<Vec<&str>>();
        // TODO Will replace with editor functionality
        let _cmdproc: () = match command_args[0] {
            "~"     => { display_help(); },
            "~save" => { save(); },
            "~show" => { show(); },
            "~open" => { println!("{}", open(command_args)); },
            _       => { println!("{}", command); }
        };
    }
}
