// main.rs - sued, the text editor of all time, short for Shut Up Editor

use rand::Rng;

fn main() {
	let messages = vec!["the editor of all time", "shut up and edit", "the nonstandard text editor", "sued as in editor, not as in law"];
	let message = messages[rand::thread_rng().gen_range(0..messages.len())];
    println!("sued - {message}");
}
