// main.rs - sued, the text editor of all time, short for Shut Up Editor

use std::io;
use rand::Rng;

fn main() {
    // Startup message
    let messages: Vec<&str> = vec!["the editor of all time", "shut up and edit", "the nonstandard text editor", "sued as in editor, not as in law"];
    let message: &str = messages[rand::thread_rng().gen_range(0..messages.len())];
    println!("sued - {message}\n`~exit` or <C-c> exits");

    // Create 'command' variable
    let mut command: String = String::new();

    // Main while loop
    while !command.eq("~exit") {
        // Clear the command variable, since read_line appends
        command.clear();

        // Read user input
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command.");

        // Remove newline from the end of the command variable,
        // since read_line adds a newline which makes sense actually
        let len: usize = command.trim_end_matches(&['\r', '\n'][..]).len();
        command.truncate(len);

        // Handle commands, which don't yet do anything
        // If a command isn't recognised, the text will
        // be println!'d instead
        // TODO Will replace with editor functionality
        let _cmdproc: () = match command.as_str() {
            "~save" => { println!("not implemented"); },
            "~open" => { println!("not implemented"); },
            _ => { println!("{}", command) }
        };
    }
}
