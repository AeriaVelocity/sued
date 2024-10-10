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
        "probably more powerful than it looks",
        "there is no visual mode",
        "the text editor that doesn't give a damn",
        "what you get is what you get",
        "what the frick is a config file",
        "less is more, much more",
        "no config file means no config bankruptcy",
        "free software, hell yeah",
        "put that mouse AWAY",
        "it looks like you're editing text, would you like help?",
        "who needs to save scripts to run them?",
        "startup_messages.push_str(&funny);",
        "there's no scripting language, if you were wondering",
    ];
    let message: &str = messages[rand::thread_rng().gen_range(0..messages.len())];
    let version = env!("CARGO_PKG_VERSION");
    println!("sued v{version} - {message}\ntype ~ for commands, otherwise just start typing");
}

/// Displays and returns the list of commands that sued supports.
/// Invoked with the `~` command.
pub fn command_list() -> Vec<String> {
    let commands = vec![
        "about",
        "clear",
        "copy",
        "correct",
        "delete",
        "exit",
        "help",
        "indent",
        "insert",
        "open",
        "prefix",
        "print",
        "prompt",
        "replace",
        "run",
        "runhere",
        "save",
        "search",
        "show",
        "substitute",
        "swap",
        "write"
    ];
    println!("{}", commands.join(", "));
    commands.into_iter().map(|s| s.to_string()).collect()
}

/// Displays a list of available commands and their descriptions.
/// Invoked with the `~help` command.
pub fn extended_command_list(prefix: &str) {
    println!("{}",
"press up and down to navigate through command history
all `range` arguments use tilde range syntax (TRS)
key: ~command arg1/alt_arg1 arg2 [optional_arg] - what the command does
~about - display about text
~clear - clear buffer
~copy [range] - copy range or whole buffer to clipboard
~correct - replace most recent line (interactive)
~delete range - immediately delete specified range of lines
~exit - exit sued
~help - display this list
~indent range level - indent a range, negative level will outdent
~insert line - insert text at specified line (interactive)
~nothing - do nothing with the buffer contents
~open [filename] - load file into buffer
~prefix [prefix] - set command prefix
~print [range] - print the contents of the buffer without line numbers
~prompt [prompt] - set input prompt
~replace line - replace specified line (interactive)
~run command - run executable or shell builtin
~runhere command - run executable or shell builtin on file contents
~save [filename] - save buffer to file
~search term - perform regex search in the whole buffer
~show [range] - display the contents of the buffer with line numbers
~substitute line pattern/replacement - perform regex substitution on the specified line
~swap source target - swap two lines
~write filename - write buffer to file without storing filename".replace("~", prefix));
}

/// Displays the sued version number and information about the editor itself.
/// Invoked with the `~about` command.
pub fn about_sued() {
    let version = env!("CARGO_PKG_VERSION");
    println!("this is sued, v{version}\n\
              sued is a vector-oriented line editor, heavily inspired by the ed editor\n\
              you can write text simply by typing, and use sued's extensive command set for editing\n\
              editor commands are prefixed with a default prefix of ~, type ~help for a full list\n\
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
pub fn show(buffer_contents: &Vec<String>, start_point: usize, end_point: usize, line_numbers: bool) {
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
            if line_numbers {
                let count: usize = start_point + index;
                let count_padded: String = format!("{:width$}", count, width = max_count_length);
                println!("{}│{}", count_padded, line);
            }
            else {
                println!("{}", line);
            }
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
            file_buffer.insert(index, input.trim_end_matches('\n').to_string());
            println!("inserted");
        }
        else {
            file_buffer.insert(index, String::new());
            println!("inserted newline");
        }
    }
}

/// A helper function for the `~replace` command.
/// Returns the number of leading spaces in the `input_str`.
fn count_leading_spaces(input_str: &str) -> usize {
    let mut count = 0;
    for c in input_str.chars() {
        if c == ' ' {
            count += 1;
        } else {
            break; // Exit the loop when a non-space character is encountered.
        }
    }
    count
}

/// Interactively replace the line at `line_number` in the `file_buffer`.
/// Provides functionality for the `~replace` and `~correct` commands.
pub fn replace(file_buffer: &mut Vec<String>, line_number: usize) {
    if check_if_line_in_buffer(file_buffer, line_number, true) {
        let original_line = file_buffer[line_number - 1].clone();
        let trimmed_line = original_line.trim();
        let leading_spaces = count_leading_spaces(&original_line);

        println!("replacing line {}", line_number);

        match leading_spaces {
            n if n >= 2 => println!("original line is '{}' (indented by {} spaces)", trimmed_line, n),
            1 => println!("original line is '{}' (indented by 1 space)", trimmed_line),
            _ => println!("original line is '{}'", trimmed_line),
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        let index = line_number - 1;
        if !input.trim().is_empty() {
            file_buffer.insert(index, input.trim_end_matches('\n').to_string());
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
pub fn copy(file_buffer: &mut Vec<String>, range: (usize, usize)) {
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
            let mut copy_message = String::new();
            if range.0 == range.1 {
                let line_number = range.0;
                if check_if_line_in_buffer(file_buffer, line_number, false) {
                    to_copy = file_buffer[line_number - 1].clone();
                    copy_message = format!("copying line {}", line_number);
                }
            }
            else {
                if check_if_line_in_buffer(file_buffer, range.0, false) && check_if_line_in_buffer(file_buffer, range.1, false) {
                    to_copy = file_buffer[range.0 - 1..range.1].join("\n");
                    copy_message = format!("copying lines {} to {}", range.0, range.1);
                }
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
                }
                else {
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
    }
    else {
        let shell = if cfg!(windows) { 
            if which("pwsh").is_ok() {
                "pwsh"
            }
            else {
                "powershell"
            }
        }
        else { 
            "sh" 
        };

        let arg = "-c";
        let command = command_args[1];

        if command == "sued" {
            editor_overflow();
            return;
        }

        match which(command) {
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
            println!("finished running {} with errors", &command);
        }
    }
}

/// Passes the current `buffer_contents` to `shell_command`.
/// Provides functionality for the `~runhere` command.
pub fn shell_command_with_file(mut command_args: Vec<String>, buffer_contents: &mut Vec<String>, file_name: Option<String>) {
    if buffer_contents.is_empty() {
        println!("no buffer contents");
    }
    else {
        let temporary_file_name: String = if file_name.is_some() {
            format!("{}", file_name.unwrap().replace(".", "-temp."))
        }
        else {
            /* Do we need a random hex string? No. Is it cool anyway? YES. */
            let hex_string: String = (0..8)
                .map(|_| {
                    let random_digit = rand::thread_rng().gen_range(0..16);
                    format!("{:x}", random_digit)
                })
                .collect();

            format!("{}.temp", hex_string)
        };

        if command_args.len() <= 1 {
            println!("run what?");
            return;
        }

        if fs::write(&temporary_file_name, buffer_contents.join("\n")).is_err() {
            println!("couldn't write temporary file");
            return;
        }

        command_args.push(temporary_file_name.clone());

        shell_command(command_args.iter().map(|s| s.as_str()).collect());

        if let Ok(new_contents) = fs::read_to_string(&temporary_file_name) {
            *buffer_contents = new_contents.lines().map(String::from).collect();
        }

        fs::remove_file(&temporary_file_name).unwrap_or_default();
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
    let mut start = 0;
    let mut escaped = false;

    for (i, c) in combined_args.char_indices() {
        if escaped {
            escaped = false;
        }
        else if c == '\\' {
            escaped = true;
        }
        else if c == '/' {
            pattern_replacement.push(&combined_args[start..i]);
            start = i + 1;
        }
    }
    
    if start <= combined_args.len() {
        pattern_replacement.push(&combined_args[start..]);
    }

    pattern_replacement
}

/// A helper function used by all commands with range specifiers.
/// Returns the range of lines to operate on.
pub fn parse_tilde_range(specifier: &str, buffer_len: usize) -> (usize, usize) {
    let start_point = 1;
    let end_point = buffer_len;

    if specifier.starts_with("~") {
        if let Ok(end) = specifier.trim_start_matches("~").parse::<usize>() {
            return (start_point, end);
        }
    }
    else if specifier.ends_with("~") {
        if let Ok(start) = specifier.trim_end_matches("~").parse::<usize>() {
            return (start, end_point);
        }
    }
    else if specifier.contains("~") {
        let range: Vec<&str> = specifier.split("~").collect();
        if let Ok(start) = range[0].parse::<usize>() {
            if let Ok(end) = range[1].parse::<usize>() {
                return (start, end);
            }
        }
    }
    else if let Ok(specified_line) = specifier.parse::<usize>() {
        return (specified_line, specified_line);
    }

    (start_point, end_point)
}

