//! Contains every function used by sued, including editing commands and helper functions.
//! 
//! This file is part of sued.
//! 
//! Visit `main.rs` for context and usage.

use std::io;
use std::fs;
use std::cmp::Ordering;
use std::path::PathBuf;
use std::process::Command;
use which::which;
use rand::Rng;
use regex::Regex;
use copypasta::{ClipboardContext, ClipboardProvider};

/// Prints a startup message with a funny joke. I hope it's funny at least.
/// Invoked at startup, obviously.
pub fn startup_message() {
    let messages: Vec<&str> = vec![
        "the shut up editor",
        "the not standard text editor",
        "it's pronounced \"soo-ed\"",
        "sued as in editor, not as in law",
        "sued, man! ~run man sued",
        "probably more powerful than it looks",
        "there is no visual mode",
        "sued: the itor fell off",
        "the text editor that doesn't give a damn",
        "what you get is what you get",
        "what the frick is a config file",
        "an inextensible, uncustomisable, free/libre text editor - and less",
        "good luck figuring out how to exit",
        "you are on your own. good luck",
        "no config file means no config bankruptcy",
        "free software, hell yeah",
        "put that mouse AWAY",
        "command history is the only nicety you get",
        "it looks like you're editing text, would you like help?",
    ];
    let message: &str = messages[rand::thread_rng().gen_range(0..messages.len())];
    let version = env!("CARGO_PKG_VERSION");
    println!("sued v{version} - {message}\ntype ~ for commands, otherwise just start typing");
}

/// Displays the list of commands that sued supports.
/// Invoked with the `~` command.
pub fn command_list() {
    println!("~about, ~clear, ~copy, ~correct, ~delete, ~exit, ~help, ~indent, ~insert, ~open, ~prompt, ~replace, ~run, ~runhere, ~save, ~search, ~show, ~substitute, ~swap, ~write");
}

/// Displays a list of available commands and their descriptions.
/// Invoked with the `~help` command.
pub fn extended_command_list() {
    println!("press up and down to navigate through command history");
    println!("~about - display about text");
    println!("~clear - clear buffer");
    println!("~copy [line] - copy line or whole buffer to clipboard");
    println!("~correct - replace most recent line (interactive)");
    println!("~delete [line] - immediately delete specified line");
    println!("~exit - exit sued");
    println!("~help - display this list");
    println!("~indent [line] [level] - indent a line, negative level will outdent");
    println!("~insert [line] - insert text at specified line (interactive)");
    println!("~open [filename] - load file into buffer");
    println!("~prompt [prompt] - set an input prompt");
    println!("~replace [line] - replace specified line (interactive)");
    println!("~run [command] - run executable or shell builtin");
    println!("~runhere [command] - run executable or shell builtin on file contents");
    println!("~save [filename] - save buffer to file");
    println!("~search [term] - perform regex search in whole buffer");
    println!("~show [start] [end] - Display the contents of the buffer.");
    println!("~substitute [line] [pattern]/[replacement] - perform regex substitution on specified line");
    println!("~swap [source] [target] - swap two lines");
    println!("~write [filename] - write buffer to file without storing filename");
}

/// Displays the sued version number and information about the editor itself.
/// Invoked with the `~about` command.
pub fn about_sued() {
    let version = env!("CARGO_PKG_VERSION");
    println!("this is sued, v{version}\n\
              sued is a vector-oriented line editor, heavily inspired by the ed editor\n\
              you can write text simply by typing, and use sued's extensive command set for editing\n\
              editor commands are prefixed with ~, type ~help for a full list\n\
              sued written by Arsalan \"Velocity\" Kazmi <sonicspeed848@gmail.com>");
}

/// Writes the `buffer_contents` to the `file_path`, if there are any contents.
/// Used to provide functionality for the `~save` command.
pub fn save(buffer_contents: &Vec<String>, file_path: &str) {
    if buffer_contents.is_empty() {
        println!("buffer empty - nothing to save");
        return;
    }

    let content = buffer_contents.join("\n");
    let path = PathBuf::from(file_path);

    match fs::write(&path, content) {
        Ok(_) => println!("saved to {}", &path.display()),
        Err(error) => eprintln!("couldn't save file to {}: {}", file_path, error),
    }
}

/// Iterates over the `buffer_contents` and displays them one by one.
/// If a range was specified, only iterate for that part.
/// Used to provide functionality for the `~show` command.
pub fn show(buffer_contents: &Vec<String>, start_point: usize, end_point: usize) {
    if buffer_contents.is_empty() {
        println!("no buffer contents");
        return;
    }
    else if !check_if_line_in_buffer(buffer_contents, start_point, false) {
        println!("invalid start point {}", start_point);
        return;
    }
    else if !check_if_line_in_buffer(buffer_contents, end_point, false) {
        println!("invalid end point {}", end_point);
        return;
    }
    else {
        let contents: Vec<String> = buffer_contents[start_point - 1..end_point].to_vec();
        let max_count_length: usize = (start_point + contents.len() - 1).to_string().len();
        for (index, line) in contents.iter().enumerate() {
            let count: usize = start_point + index;
            let count_padded: String = format!("{:width$}", count, width = max_count_length);
            println!("{}│{}", count_padded, line);
        }
    }
}

/// Verifies the `file_path`'s file existence, then returns the file contents as a `String` vector.
/// Used for the `~open` command.
pub fn open(file_path: &str, current_file_path: &mut Option<String>) -> Vec<String> {
    let file_exists = fs::read_to_string(file_path);
    match file_exists {
        Ok(contents) => {
            println!("file {} opened", file_path);
            *current_file_path = Some(file_path.to_string());
            return contents.lines().map(|line| line.to_owned()).collect();
        }
        Err(_) => {
            println!("no such file {}", file_path);
        }
    }
    Vec::new()
}

/// Checks if a given `line_number` is in the `file_buffer`.
/// Used by `insert`, `replace`, `swap` and `delete`.
fn check_if_line_in_buffer(file_buffer: &Vec<String>, line_number: usize, verbose: bool) -> bool {
    if line_number < 1 {
        if verbose {
            println!("invalid line {}", line_number);
        }
        return false;
    }

    if file_buffer.is_empty() {
        if verbose {
            println!("no buffer contents");
        }
        return false;
    }

    if line_number <= file_buffer.len() {
        return true;
    }

    if verbose {
        println!("no line {}", line_number);
    }

    false
}

/// Interactively insert a line at `line_number` in the `file_buffer`.
/// Provides functionality for the `~insert` command.
pub fn insert(file_buffer: &mut Vec<String>, line_number: usize) {
    if check_if_line_in_buffer(file_buffer, line_number, true) {
        println!("inserting into line {}", line_number);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        let index = line_number - 1;
        if !input.trim().is_empty() {
            file_buffer.insert(index, input.trim().to_string());
            println!("inserted");
        }
        else {
            file_buffer.insert(index, String::new());
            println!("inserted newline");
        }
    }
}

/// Interactively replace the line at `line_number` in the `file_buffer`.
/// Provides functionality for the `~replace` command.
pub fn replace(file_buffer: &mut Vec<String>, line_number: usize) {
    if check_if_line_in_buffer(file_buffer, line_number, true) {
        println!("replacing line {}", line_number);
        println!("original line is {}", file_buffer[line_number - 1]);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        let index = line_number - 1;
        if !input.trim().is_empty() {
            file_buffer.insert(index, input.trim().to_string());
            file_buffer.remove(index + 1);
            println!("replaced");
        }
        else {
            println!("replace cancelled; try ~delete if you wanted that instead");
        }
    }
}

/// Swap the `source_line` with the `target_line` in the `file_buffer`.
/// Provides functionality for the `~swap` command.
pub fn swap(file_buffer: &mut Vec<String>, source_line: usize, target_line: usize) {
    if check_if_line_in_buffer(file_buffer, source_line, true) && check_if_line_in_buffer(file_buffer, target_line, true) {
        if source_line == target_line {
            println!("lines are the same");
            return;
        }

        let source_index = source_line - 1;
        let target_index = target_line - 1;

        let line = file_buffer.remove(source_index);
        file_buffer.insert(target_index, line);
    }
}

/// Remove the `line_number` from the `file_buffer`.
/// Provides functionality for the `~delete` command.
pub fn delete(file_buffer: &mut Vec<String>, line_number: usize) {
    if check_if_line_in_buffer(file_buffer, line_number, true) {
        file_buffer.remove(line_number - 1);
    }
}

/// Copy the provided `line_number` to the system clipboard.
/// If no `line_number` is specified (it's not in the buffer), copy the whole buffer.
/// Provides functionality for the `~copy` command.
pub fn copy(file_buffer: &mut Vec<String>, line_number: usize) {
    if file_buffer.is_empty() {
        println!("no buffer contents");
        return;
    }
    #[cfg(any(target_os = "android", target_os = "ios"))] {
        println!("~copy is unsupported on your device, sorry");
        return;
    }
    let mut clipboard_context = ClipboardContext::new().unwrap();
    let file_contents = file_buffer.join("\n");
    let mut to_copy = file_contents;

    match clipboard_context.get_contents() {
        Ok(_) => {
            let mut copy_message = "copying whole buffer".to_string();
            if check_if_line_in_buffer(file_buffer, line_number, false) {
                to_copy = file_buffer[line_number - 1].clone();
                copy_message = format!("copying line {}", line_number);
            }
            println!("{}", copy_message);
            clipboard_context.set_contents(to_copy).unwrap();
        }
        Err(e) => println!("copy failed, because {}", e),
    }
}

/// Perform a regex `replace()` on `line_number`, with the `pattern` and `replacement`.
/// Provides functionality for the `~substitute` command.
pub fn substitute(file_buffer: &mut Vec<String>, line_number: usize, pattern: &str, replacement: &str) {
    if check_if_line_in_buffer(file_buffer, line_number, true) {
        let index = line_number - 1;
        let line = &mut file_buffer[index];
        match Regex::new(pattern) {
            Ok(re) => {
                let replaced_line = re.replace(line, replacement).to_string();
                *line = replaced_line;
            },
            Err(e) => {
                let error_message = e.to_string();
                let lines: Vec<String> = error_message.lines().map(String::from).collect();

                if let Some(error) = lines.last() {
                    println!("substitute failed, because {}", error.to_lowercase().replace("error: ", ""));
                } else {
                    println!("substitute failed, for some reason");
                }
            },
        }
    }
}

/// Searches for the given `term` in the `file_buffer` and prints matching lines.
/// Provides functionality for the `~search` command.
pub fn search(file_buffer: &[String], term: &str) {
    let escaped_term = regex::escape(term);

    let regex = Regex::new(escaped_term.as_str()).unwrap();

    for (line_number, line) in file_buffer.iter().enumerate() {
        if regex.is_match(line) {
            println!("line {}: {}", line_number + 1, line);
        }
    }
}

/// Run a shell command with the OS shell, and fall back to a shell built-in if it fails.
/// Provides functionality for the `~run` command.
pub fn shell_command(mut command_args: Vec<&str>) {
    if command_args.len() <= 1 {
        println!("run what?");
    } else {
        let shell = if cfg!(windows) { 
            "cmd"
        }
        else { 
            "sh" 
        };

        let arg = if cfg!(windows) {
            "/c"
        }
        else { 
            "-c"
        };

        let command = command_args[1];

        if command == "sued" {
            editor_overflow();
            return;
        }

        match which(&command) {
            Ok(path) => println!("running {}", path.to_string_lossy()),
            Err(_) => println!("{} wasn't found; trying to run it anyway", &command)
        }

        command_args.drain(0..1);

        let cmd = Command::new(shell)
            .arg(arg)
            .arg(command_args.join(" "))
            .status()
            .expect("command failed");

        if cmd.success() {
            println!("finished running {}", &command);
        }
        else {
            println!("failed to run {}", &command);
        }
    }
}

/// Passes the current `buffer_contents` to `shell_command`.
/// Provides functionality for the `~runhere` command.
pub fn shell_command_with_file(mut command_args: Vec<String>, buffer_contents: &mut Vec<String>) {
    if buffer_contents.is_empty() {
        println!("no buffer contents");
    } else {
        /* Do we need a random hex string? No. Is it cool anyway? YES. */
        let hex_string: String = (0..8)
            .map(|_| {
                let random_digit = rand::thread_rng().gen_range(0..16);
                format!("{:x}", random_digit)
            })
            .collect();

        let file_name: String = format!("{}.temp", hex_string);

        if command_args.len() <= 1 {
            println!("run what?");
            return;
        }

        if fs::write(&file_name, buffer_contents.join("\n")).is_err() {
            println!("couldn't write temporary file");
            return;
        }

        command_args.push(file_name.clone());

        shell_command(command_args.iter().map(|s| s.as_str()).collect());

        if let Ok(new_contents) = fs::read_to_string(&file_name) {
            *buffer_contents = new_contents.lines().map(String::from).collect();
        }

        fs::remove_file(&file_name).unwrap_or_default();
    }
}

/// Indent the line at `line_number` by `indentation` spaces.
/// Used for the `~indent` command.
pub fn indent(file_buffer: &mut Vec<String>, line_number: usize, indentation: isize) {
    if check_if_line_in_buffer(file_buffer, line_number, true) {
        let index = line_number - 1;
        let line = &mut file_buffer[index];
        match indentation.cmp(&0) {
            Ordering::Greater => {
                let indented_line = format!("{:indent$}{}", "", line, indent = indentation as usize);
                *line = indented_line;
            }
            Ordering::Less => {
                let line_len = line.len() as isize;
                let new_len = (line_len + indentation).max(0) as usize;
                let indented_line = format!("{:indent$}", &line[line_len as usize - new_len..], indent = new_len);
                *line = indented_line;
            }
            _ => println!("invalid indent level"),
        }
    }
}

/// Displays a Blue Screen of Death-like error message.
/// Technically I don't need it, but it's funny.
pub fn crash(error_code: &str, hex_codes: &[u32]) {
    let mut populated_hex_codes = [0x00000000; 4];
    let num_values = hex_codes.len().min(4);
    populated_hex_codes[..num_values].copy_from_slice(&hex_codes[..num_values]);

    eprintln!("stop: {}: 0x{:08X} (0x{:08X},0x{:08X},0x{:08X})",
              error_code.to_uppercase(),
              populated_hex_codes[0],
              populated_hex_codes[1],
              populated_hex_codes[2],
              populated_hex_codes[3],
    );
    std::process::exit(1);
}

/// A joke function that simulates an "editor overflow" error.
/// Invoked with `~run sued`.
pub fn editor_overflow() {
    loop {
        eprintln!("editor overflow"); 
        eprintln!("(a)bort, (r)etry, (f)ail?"); 
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "a" => {
                println!("let us never speak of this again");
                break;
            },
            "f" => {
                crash("editor_overflow", &[0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF])
            },
            _ => ()
        }
    }
}

/// A nothing function that does nothing.
/// Used to provide functionality for the `~nothing` command.
pub fn nothing(file_buffer: &Vec<String>) {
    if file_buffer.is_empty() {
        println!("no buffer contents");
    }
    let buffer_contents: String = file_buffer.join("; ");
    println!("doing nothing with {}", buffer_contents);
}

/// A helper function used for the ~substitute command.
pub fn split_pattern_replacement(combined_args: &str) -> Vec<&str> {
    let mut pattern_replacement = Vec::new();
    let mut temp_str = combined_args;
    let mut previous_char: Option<char> = None;

    for (i, c) in combined_args.char_indices() {
        if previous_char == Some('\\') {
            // Previous character is a backslash, continue to the next character
            previous_char = None;
        }
        else if c == '/' {
            if previous_char == Some('\\') {
                // Special case: `\/` should be treated as a single `/` and included in the first element
                previous_char = Some(c);
            }
            else {
                // Found a forward slash, push the accumulated string to the result and reset
                pattern_replacement.push(&temp_str[..i]);
                temp_str = &combined_args[i + 1..];
                previous_char = Some(c);
            }
        } else {
            // Any other character, update the previous character
            previous_char = Some(c);
        }
    }
    // Push the remaining string to the result
    if !temp_str.is_empty() {
        pattern_replacement.push(temp_str);
    }
    pattern_replacement
}
