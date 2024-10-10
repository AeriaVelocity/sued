//! sued - shut up editor, a vector-oriented line editor by Arsalan "Aeria" Kazmi
//! 
//! to understand sued, read `README.md` or `https://aeriavelocity.github.io/sued`.
//! 
//! sued is free software licensed under the WTFPL.

use std::env;
use shellexpand::tilde;
use linefeed::{Interface, ReadResult};

// Please see the corresponding `functions.rs` file for those definitions.
mod functions;
use functions as suedfn;

/// This struct is used to represent the file buffer.
/// `contents` will contain the text contents of the file as a Vec,
/// and `file_path` will, obviously, contain the file path.
struct FileBuffer {
    contents: Vec<String>,
    file_path: Option<String>,
}

impl std::fmt::Display for FileBuffer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.contents.join("\n"))
    }
}

enum ExitStatus {
    Success,
    Failure,
}

/// It's the main function.
/// I don't know what you expected.
fn main() {
    suedfn::startup_message();

    let interface = Interface::new("sued").unwrap();
    let mut buffer = FileBuffer {
        contents: Vec::new(),
        file_path: None,
    };

    let mut prompt = String::new();
    let mut prefix = String::from("~");

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        buffer.contents = suedfn::open(&args[1], &mut buffer.file_path);
        buffer.file_path = Some(args[1].clone());
    }

    ctrlc::set_handler(|| {
        println!("use ~exit to exit sued");
    })
    .expect("Error setting Ctrl-C handler");

    while let ReadResult::Input(line) = interface.read_line().unwrap() {
        let command = line.trim_end().to_string();
        interface.add_history_unique(command.clone());
        let command_args = command.split(' ').collect::<Vec<&str>>();
        if command_args[0] == prefix {
            suedfn::command_list();
        }
        else if command.starts_with(&prefix) {
            if let ExitStatus::Failure = process_command(command_args, &mut buffer, &mut prompt, &mut prefix){
                break;
            }
        }
        else {
            let to_write = command_args.clone().join(" ");
            buffer.contents.push(to_write);
        }
        interface.set_prompt(&prompt).unwrap_or_default();
    }
}

/// Process an editing command passed from `command_args`.
/// Requires mutable access to `buffer`, `prompt` and `prefix`, since this function will need to modify these.
/// Related functions are available in `functions.rs`.
fn process_command(command_args: Vec<&str>, buffer: &mut FileBuffer, prompt: &mut String, prefix: &mut String) -> ExitStatus {
    match command_args[0].to_lowercase().replace(prefix.as_str(), "").as_str() {
        // Help commands
        "about" => { suedfn::about_sued(); },
        "help" => { suedfn::extended_command_list(&prefix); },

        // Buffer manipulation
        "clear" => { 
            buffer.contents.clear();
            buffer.file_path = None;
        },
        "copy" => {
            if command_args.len() >= 2 {
                let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                suedfn::copy(&mut buffer.contents, line_number);
            }
            else {
                suedfn::copy(&mut buffer.contents, 0);
            }
        }
        "correct" => {
            let line_number = buffer.contents.len();
            suedfn::replace(&mut buffer.contents, line_number);
        }
        "del" | "delete" => {
            if command_args.len() >= 3 {
                let start = command_args[1].parse::<usize>().unwrap_or_default();
                let end = command_args[2].parse::<usize>().unwrap_or_default();
                suedfn::delete_range(&mut buffer.contents, start, end);
            }
            else if command_args.len() >= 2 {
                let line_number = command_args[1].parse::<usize>().unwrap_or_default();
                suedfn::delete(&mut buffer.contents, line_number);
            }
            else {
                println!("delete which line?");
            }
        }
        "indent" => {
            if command_args.len() >= 2 {
                let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                if command_args.len() >= 3 {
                    let indentation: isize = command_args[2].parse().unwrap_or(0);
                    suedfn::indent(&mut buffer.contents, line_number, indentation);
                }
                else {
                    println!("indent line {} by how many spaces?", line_number);
                }
            }
            else {
                println!("indent which line?");
            }
        },
        "insert" => {
            if command_args.len() >= 2 {
                let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                suedfn::insert(&mut buffer.contents, line_number);
            }
            else {
                println!("insert where?");
            }
        },
        "open" => { 
            if command_args.len() >= 2 {
                let file_name_with_spaces = command_args[1..].join(" ");
                let expanded_file_path = tilde(&file_name_with_spaces).to_string();
                buffer.contents = suedfn::open(expanded_file_path.as_str(), &mut buffer.file_path);
            }
            else {
                println!("open what?");
            }
        },
        "replace" => {
            if command_args.len() >= 2 {
                let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                suedfn::replace(&mut buffer.contents, line_number);
            }
            else {
                println!("replace which line?");
            }
        },
        "save" => {
            let mut destination: String = buffer.file_path.clone().unwrap_or_default();

            if command_args.len() >= 2 {
                destination = command_args[1..].join(" ");
            }

            let expanded_file_path: String = tilde(&destination).to_string();

            if !expanded_file_path.trim().is_empty() {
                suedfn::save(&buffer.contents, expanded_file_path.as_str());
                buffer.file_path = Some(destination);
            }
            else {
                println!("save where?");
            }
        },
        "sub" | "substitute" => {
            if command_args.len() >= 3 {
                let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                let combined_args = command_args[2..].join(" ");
                let pattern_replacement = suedfn::split_pattern_replacement(combined_args.as_str());
                if pattern_replacement.len() >= 2 {
                    let pattern = pattern_replacement[0];
                    let replacement = pattern_replacement[1];
                    suedfn::substitute(&mut buffer.contents, line_number, pattern, replacement);
                }
                else {
                    println!("substitute what?");
                    println!("try substitute line pattern/replacement");
                }
            }
            else if command_args.len() >= 2 {
                println!("substitute what?");
                println!("try substitute line pattern/replacement");
            }
            else {
                println!("substitute which line?");
            }
        }
        "swap" => {
            if command_args.len() >= 3 {
                let source_line = command_args[1].parse::<usize>().unwrap_or(0);
                let target_line = command_args[2].parse::<usize>().unwrap_or(0);
                suedfn::swap(&mut buffer.contents, source_line, target_line);
            }
            else if command_args.len() >= 2 {
                println!("swap line {} with what?", command_args[1]);
            }
            else {
                println!("swap which lines?");
            }
        },
        "write" => {
            let mut destination: String = buffer.file_path.clone().unwrap_or_default();

            if command_args.len() >= 2 {
                destination = command_args[1..].join(" ");
            }

            let expanded_file_path: String = tilde(&destination).to_string();

            if !expanded_file_path.trim().is_empty() {
                suedfn::save(&buffer.contents, expanded_file_path.as_str());
            }
            else {
                println!("write where?");
            }
        },

        // Informational commands
        "search" => {
            if command_args.len() >= 2 {
                let term = command_args[1..].join(" ");
                suedfn::search(&buffer.contents, &term);
            }
            else {
                println!("search for what?");
            }
        },
        "print" => {
            let mut start_point = 1;
            let mut end_point = buffer.contents.len();

            // If the argument is prefixed with `~`, use the range from `start_point` to arg1
            if command_args.len() >= 2 && command_args[1].starts_with("~") {
                let start_from_arg = command_args[1].replace("~", "");
                if let Ok(start_from_arg) = start_from_arg.parse::<usize>() {
                    end_point = start_from_arg;
                }
            }

            // Do the opposite if the argument is suffixed with `~`
            else if command_args.len() >= 2 && command_args[1].ends_with("~") {
                let end_from_arg = command_args[1].replace("~", "");
                if let Ok(end_from_arg) = end_from_arg.parse::<usize>() {
                    start_point = end_from_arg;
                }
            }

            // If a range argument (format X~Y), display the range from X to Y
            else if command_args.len() >= 2 && command_args[1].contains("~") {
                let range: Vec<&str> = command_args[1].split("~").collect();
                if let Ok(start_from_arg) = range[0].parse::<usize>() {
                    start_point = start_from_arg;
                }
                if let Ok(end_from_arg) = range[1].parse::<usize>() {
                    end_point = end_from_arg;
                }
            }

            // If only one argument, then set the start point and end point to the same value
            else if command_args.len() == 2 {
                if let Ok(start_from_arg) = command_args[1].parse::<usize>() {
                    start_point = start_from_arg;
                    end_point = start_from_arg;
                }
            }

            suedfn::show(&buffer.contents, start_point, end_point, false);
        },
        "show" => {
            let mut start_point = 1;
            let mut end_point = buffer.contents.len();

            // If the argument is prefixed with `~`, use the range from `start_point` to arg1
            if command_args.len() >= 2 && command_args[1].starts_with("~") {
                let start_from_arg = command_args[1].replace("~", "");
                if let Ok(start_from_arg) = start_from_arg.parse::<usize>() {
                    end_point = start_from_arg;
                }
            }

            // Do the opposite if the argument is suffixed with `~`
            else if command_args.len() >= 2 && command_args[1].ends_with("~") {
                let end_from_arg = command_args[1].replace("~", "");
                if let Ok(end_from_arg) = end_from_arg.parse::<usize>() {
                    start_point = end_from_arg;
                }
            }

            // If a range argument (format X~Y), display the range from X to Y
            else if command_args.len() >= 2 && command_args[1].contains("~") {
                let range: Vec<&str> = command_args[1].split("~").collect();
                if let Ok(start_from_arg) = range[0].parse::<usize>() {
                    start_point = start_from_arg;
                }
                if let Ok(end_from_arg) = range[1].parse::<usize>() {
                    end_point = end_from_arg;
                }
            }

            // If only one argument, then set the start point and end point to the same value
            else if command_args.len() == 2 {
                if let Ok(start_from_arg) = command_args[1].parse::<usize>() {
                    start_point = start_from_arg;
                    end_point = start_from_arg;
                }
            }

            suedfn::show(&buffer.contents, start_point, end_point, true);
        },
        
        // Miscellaneous commands
        "bsod" => { suedfn::crash("USER_IS_STUPID", &[0x0000DEAD, 0x00000101, 0xFFFFFFFF, 56]); },
        "prefix" => {
            prefix.clear();
            if command_args.len() < 2 {
                prefix.push_str("~");
                println!("prefix reset to ~, try passing a prefix if you wanted that instead");
            }
            else {
                let new_prefix = command_args[1];
                prefix.push_str(new_prefix);
            }
        }
        "prompt" => {
            prompt.clear();
            if command_args.len() < 2 {
                println!("prompt reset, try passing a prompt if you wanted that instead");
            }
            else {
                let new_prompt = format!("{} ", command_args[1..].join(" "));
                prompt.push_str(&new_prompt);
            }
        }
        "run"  => { suedfn::shell_command(command_args.clone()); },
        "runhere" => { 
            let command_args_string = command_args.iter().map(|&s| s.to_string()).collect();
            suedfn::shell_command_with_file(command_args_string, &mut buffer.contents, buffer.file_path.clone());
        }
        "nothing" => { suedfn::nothing(&buffer.contents); },

        // Exit command
        "exit" | "quit" => return ExitStatus::Failure,

        // Fallback
        _ => { 
            println!("{} is an unknown command", command_args[0].replace(prefix.as_str(), ""));
        }
    };
    ExitStatus::Success
}
