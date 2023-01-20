// main.rs - sued, the text editor of all time, short for Shut Up Editor

use std::io;
use rand::Rng;

fn main() {
    // Startup message
    let messages = vec!["the editor of all time", "shut up and edit", "the nonstandard text editor", "sued as in editor, not as in law"];
    let message = messages[rand::thread_rng().gen_range(0..messages.len())];
    println!("sued - {message}\n`~exit` or <C-c> exits");

    // Create 'command' variable
    let mut command = String::new();

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
        let len = command.trim_end_matches(&['\r', '\n'][..]).len();
        command.truncate(len);

        // Print the command entered, for debug purposes
        // TODO Add editor commands
        // TODO Will replace with editor functionality
        println!("{}", command);
    }
}
