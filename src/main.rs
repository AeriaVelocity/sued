/// sued - a vector-oriented line editor that doesn't give a damn
/// to understand sued, read `README.md` or `that1m8head.github.io/sued`
/// sued is free software licensed under the WTFPL

use std::io;
use std::fs;
use std::env;
use std::cmp::Ordering;
use std::path::PathBuf;
use std::process::Command;
use which::which;
use rand::Rng;
use shellexpand::tilde;
use regex::Regex;
use copypasta::{ClipboardContext, ClipboardProvider};
use linefeed::{Interface, ReadResult};

/// Prints a startup message with a funny joke. I hope it's funny at least.
/// Invoked at startup, obviously.
fn startup_message() {
    let messages: Vec<&str> = vec![
        "the editor of all time",
        "shut up and edit",
        "the nonstandard text editor",
        "it's pronounced \"soo-ed\"",
        "sued as in editor, not as in law",
        "sued, man! ~run man sued",
        "there is no visual mode",
        "the itor fell off",
        "the ultimate blank slate",
        "words matter; nothing else does",
        "the text editor that doesn't give a damn",
        "write like no one is watching, because they're not",
        "syntax? never heard of them",
        "what you get is what you get",
        "what the frick is a config file",
        "a non-extensible, uncustomisable but still free/libre editor",
        "text is stored in the balls",
        "want to configure? learn rust",
        "good luck figuring out how to exit",
        "sublime is temporary, sued is eternal",
        "you are on your own. good luck",
        "back in the day they charged for stuff like this",
        "no cursor keys, no need to worry about emacs pinky",
        "the control key is only used in emergencies",
        "no need for an evil-mode, sued is evil enough",
        "no config file means no config bankruptcy",
        "if vim is evil, sued is demonic",
        "free software, hell yeah",
    ];
    let message: &str = messages[rand::thread_rng().gen_range(0..messages.len())];
    let version = env!("CARGO_PKG_VERSION");
    println!("sued v{version} - {message}\ntype ~ for commands, otherwise just start typing");
    
}

/// Displays the list of commands that sued supports.
/// Invoked with the `~` command.
fn command_list() {
    println!("~about, ~clear, ~copy, ~correct, ~delete, ~exit, ~help, ~indent, ~insert, ~open, ~replace, ~run, ~save, ~search, ~show, ~substitute, ~swap, ~write");
}

/// Displays a list of available commands and their descriptions.
/// Invoked with the `~help` command.
fn extended_command_list() {
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
    println!("~replace [line] - replace specified line (interactive)");
    println!("~run [command] - run executable or shell builtin");
    println!("~save [filename] - save buffer to file");
    println!("~search [term] - perform regex search in whole buffer");
    println!("~show [start] [end] - Display the contents of the buffer.");
    println!("~substitute [line] [pattern]/[replacement] - perform regex substitution on specified line");
    println!("~swap [source] [target] - swap two lines");
    println!("~write [filename] - write buffer to file without storing filename");
}

/// Displays the sued version number and information about the editor itself.
/// Invoked with the `~about` command.
fn about_sued() {
    let version = env!("CARGO_PKG_VERSION");
    println!("this is sued, v{version}\n\
              sued is a vector-oriented line editor, like ed but also not at all\n\
              to write stuff, just start typing after the welcome message\n\
              editor commands are prefixed with ~ (for example ~exit to quit the editor)\n\
              there's no syntax highlighting or anything like that. you just write\n\
              sued written by Arsalan Kazmi (That1M8Head)");
}

/// Writes the `buffer_contents` to the `file_path`, if there are any contents.
/// Used to provide functionality for the `~save` command.
fn save(buffer_contents: &Vec<String>, file_path: &str) {
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
fn show(buffer_contents: &Vec<String>, start_point: usize, end_point: usize) {
    if buffer_contents.is_empty() {
        println!("no buffer contents");
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
fn open(file_path: &str, current_file_path: &mut Option<String>) -> Vec<String> {
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
fn insert(file_buffer: &mut Vec<String>, line_number: usize) {
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
            println!("insert cancelled");
        }
    }
}

/// Interactively replace the line at `line_number` in the `file_buffer`.
/// Provides functionality for the `~replace` command.
fn replace(file_buffer: &mut Vec<String>, line_number: usize) {
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
            println!("replace cancelled");
        }
    }
}

/// Swap the `source_line` with the `target_line` in the `file_buffer`.
/// Provides functionality for the `~swap` command.
fn swap(file_buffer: &mut Vec<String>, source_line: usize, target_line: usize) {
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
fn delete(file_buffer: &mut Vec<String>, line_number: usize) {
    if check_if_line_in_buffer(file_buffer, line_number, true) {
        file_buffer.remove(line_number - 1);
    }
}

/// Copy the provided `line_number` to the system clipboard.
/// If no `line_number` is specified (it's not in the buffer), copy the whole buffer.
/// Provides functionality for the `~copy` command.
fn copy(file_buffer: &mut Vec<String>, line_number: usize) {
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
fn substitute(file_buffer: &mut Vec<String>, line_number: usize, pattern: &str, replacement: &str) {
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
fn search(file_buffer: &[String], term: &str) {
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
fn shell_command(mut command_args: Vec<&str>) {
    if command_args.len() <= 1 {
        println!("run what?");
    } else {
        let command = command_args[1];
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
        if command == "sued" {
            editor_overflow();
            return;
        }
        match which(command) {
            Ok(path) => println!("running {}", path.to_string_lossy()),
            Err(_) => println!("{} wasn't found; trying to run it anyway", command)
        }
        command_args.drain(0..2);
        let cmd = Command::new(shell)
            .arg(arg)
            .arg(command)
            .args(command_args)
            .status()
            .expect("command failed");
        if cmd.success() {
            println!("finished running {}", command);
        }
        else {
            println!("failed to run {}", command);
        }
    }
}

/// Indent the line at `line_number` by `indentation` spaces.
/// Used for the `~indent` command.
fn indent(file_buffer: &mut Vec<String>, line_number: usize, indentation: isize) {
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
fn crash(error_code: &str, hex_codes: &[u32]) {
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
fn editor_overflow() {
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
fn nothing(file_buffer: &Vec<String>) {
    if file_buffer.is_empty() {
        println!("no buffer contents");
    }
    let buffer_contents: String = file_buffer.join("; ");
    println!("doing nothing with {}", buffer_contents);
}

/// A helper function used for the ~substitute command.
fn split_pattern_replacement(combined_args: &str) -> Vec<&str> {
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

/// It's the main function.
/// I don't know what you expected.
fn main() {
    startup_message();
    let interface = Interface::new("sued").unwrap();
    let mut file_buffer: Vec<String> = Vec::new();
    let mut file_path: Option<String> = None;
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        file_buffer = open(&args[1], &mut file_path);
    }
    while let ReadResult::Input(line) = interface.read_line().unwrap() {
        let command = line.trim_end().to_string();
        interface.add_history_unique(command.clone());
        let command_args = command.split(' ').collect::<Vec<&str>>();
        match command_args[0].to_lowercase().as_str() {
            // Help commands
            "~"     => { command_list(); },
            "~about" => { about_sued(); },
            "~help" => { extended_command_list(); },

            // Buffer manipulation
            "~clear" => { 
                file_buffer.clear();
                file_path = None;
            },
            "~copy" => {
                if command_args.len() >= 2 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    copy(&mut file_buffer, line_number);
                }
                else {
                    copy(&mut file_buffer, 0);
                }
            }
            "~correct" => {
                let line_number = file_buffer.len();
                replace(&mut file_buffer, line_number);
            }
            "~del" | "~delete" => {
                if command_args.len() >= 2 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    delete(&mut file_buffer, line_number);
                }
                else {
                    println!("delete which line?");
                }
            }
            "~indent" => {
                if command_args.len() >= 2 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    if command_args.len() >= 3 {
                        let indentation: isize = command_args[2].parse().unwrap_or(0);
                        indent(&mut file_buffer, line_number, indentation);
                    }
                    else {
                        println!("indent line {} how?", line_number);
                    }
                }
                else {
                    println!("indent which line?");
                }
            },
            "~insert" => {
                if command_args.len() >= 2 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    insert(&mut file_buffer, line_number);
                } else {
                    println!("insert where?");
                }
            },
            "~open" => { 
                if command_args.len() >= 2 {
                    let file_name_with_spaces = command_args[1..].join(" ");
                    let expanded_file_path = tilde(&file_name_with_spaces).to_string();
                    file_buffer = open(expanded_file_path.as_str(), &mut file_path);
                }
                else {
                    println!("open what?");
                }
            },
            "~replace" => {
                if command_args.len() >= 2 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    replace(&mut file_buffer, line_number);
                } else {
                    println!("replace which line?");
                }
            },
            "~save" => {
                let mut destination: String = file_path.clone().unwrap_or_default();

                if command_args.len() >= 2 {
                    destination = command_args[1..].join(" ");
                }

                let expanded_file_path: String = tilde(&destination).to_string();

                if !expanded_file_path.trim().is_empty() {
                    save(&file_buffer, expanded_file_path.as_str());
                    file_path = Some(destination);
                }
                else {
                    println!("save where?");
                }
            },
            "~sub" | "~substitute" => {
                if command_args.len() >= 3 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    let combined_args = command_args[2..].join(" ");
                    let pattern_replacement = split_pattern_replacement(combined_args.as_str());
                    if pattern_replacement.len() >= 2 {
                        let pattern = pattern_replacement[0];
                        let replacement = pattern_replacement[1];
                        substitute(&mut file_buffer, line_number, pattern, replacement);
                    }
                    else {
                        println!("substitute what?");
                        println!("try ~substitute line pattern/replacement");
                    }
                }
                else if command_args.len() >= 2 {
                    println!("substitute what?");
                    println!("try ~substitute line pattern/replacement");
                }
                else {
                    println!("substitute which line?");
                }
            }
            "~swap" => {
                if command_args.len() >= 3 {
                    let source_line = command_args[1].parse::<usize>().unwrap_or(0);
                    let target_line = command_args[2].parse::<usize>().unwrap_or(0);
                    swap(&mut file_buffer, source_line, target_line);
                }
                else if command_args.len() >= 2 {
                    println!("swap line {} with what?", command_args[1]);
                }
                else {
                    println!("swap which lines?");
                }
            },
            "~write" => {
                let mut destination: String = file_path.clone().unwrap_or_default();

                if command_args.len() >= 2 {
                    destination = command_args[1..].join(" ");
                }

                let expanded_file_path: String = tilde(&destination).to_string();

                if !expanded_file_path.trim().is_empty() {
                    save(&file_buffer, expanded_file_path.as_str());
                }
                else {
                    println!("write where?");
                }
            },

            // Informational commands
            "~search" => {
                if command_args.len() >= 2 {
                    let term = command_args[1..].join(" ");
                    search(&file_buffer, &term);
                } else {
                    println!("search for what?");
                }
            },
            "~show" => {
                let mut start_point = 1;
                let mut end_point = file_buffer.len();
                if command_args.len() >= 2 {
                    if let Ok(start_from_arg) = command_args[1].parse::<usize>() {
                        start_point = start_from_arg;
                    }
                    else {
                        println!("invalid start range");
                    }
                }
                if command_args.len() >= 3 {
                    if let Ok(end_from_arg) = command_args[2].parse::<usize>() {
                        end_point = end_from_arg;
                    }
                    else {
                        println!("invalid end range");
                    }
                }
                show(&file_buffer, start_point, end_point);
            },
            
            // Miscellaneous commands
            "~bsod" => { crash("USER_IS_STUPID", &[0x0000DEAD, 0x00000101, 0xFFFFFFFF, 56]); },
            "~run"  => { shell_command(command_args.clone()); },
            "~nothing" => { nothing(&file_buffer); },

            // Exit command
            "~exit" => break,

            // Fallback
            _       => { 
                if command_args[0].starts_with('~') {
                    println!("{} is an unknown command", command_args[0]);
                }
                else {
                    let to_write = command.clone();
                    file_buffer.push(to_write);
                }
            }
        };
    }
}
