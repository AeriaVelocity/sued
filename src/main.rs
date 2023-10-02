//! sued - shut up editor, a vector-oriented line editor.
//! 
//! to understand sued, read `README.md` or `that1m8head.github.io/sued`.
//! 
//! sued is free software licensed under the WTFPL.

use std::env;
use shellexpand::tilde;
use linefeed::{Interface, ReadResult};

// Please see the corresponding `functions.rs` file for those definitions.
mod functions;
use functions::*;

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

/// It's the main function.
/// I don't know what you expected.
fn main() {
    startup_message();

    let interface = Interface::new("sued").unwrap();
    let mut buffer = FileBuffer {
        contents: Vec::new(),
        file_path: None,
    };

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        buffer.contents = open(&args[1], &mut buffer.file_path);
        buffer.file_path = Some(args[1].clone());
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
                buffer.contents.clear();
                buffer.file_path = None;
            },
            "~copy" => {
                if command_args.len() >= 2 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    copy(&mut buffer.contents, line_number);
                }
                else {
                    copy(&mut buffer.contents, 0);
                }
            }
            "~correct" => {
                let line_number = buffer.contents.len();
                replace(&mut buffer.contents, line_number);
            }
            "~del" | "~delete" => {
                if command_args.len() >= 2 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    delete(&mut buffer.contents, line_number);
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
                        indent(&mut buffer.contents, line_number, indentation);
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
                    insert(&mut buffer.contents, line_number);
                } else {
                    println!("insert where?");
                }
            },
            "~open" => { 
                if command_args.len() >= 2 {
                    let file_name_with_spaces = command_args[1..].join(" ");
                    let expanded_file_path = tilde(&file_name_with_spaces).to_string();
                    buffer.contents = open(expanded_file_path.as_str(), &mut buffer.file_path);
                }
                else {
                    println!("open what?");
                }
            },
            "~replace" => {
                if command_args.len() >= 2 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    replace(&mut buffer.contents, line_number);
                } else {
                    println!("replace which line?");
                }
            },
            "~save" => {
                let mut destination: String = buffer.file_path.clone().unwrap_or_default();

                if command_args.len() >= 2 {
                    destination = command_args[1..].join(" ");
                }

                let expanded_file_path: String = tilde(&destination).to_string();

                if !expanded_file_path.trim().is_empty() {
                    save(&buffer.contents, expanded_file_path.as_str());
                    buffer.file_path = Some(destination);
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
                        substitute(&mut buffer.contents, line_number, pattern, replacement);
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
                    swap(&mut buffer.contents, source_line, target_line);
                }
                else if command_args.len() >= 2 {
                    println!("swap line {} with what?", command_args[1]);
                }
                else {
                    println!("swap which lines?");
                }
            },
            "~write" => {
                let mut destination: String = buffer.file_path.clone().unwrap_or_default();

                if command_args.len() >= 2 {
                    destination = command_args[1..].join(" ");
                }

                let expanded_file_path: String = tilde(&destination).to_string();

                if !expanded_file_path.trim().is_empty() {
                    save(&buffer.contents, expanded_file_path.as_str());
                }
                else {
                    println!("write where?");
                }
            },

            // Informational commands
            "~search" => {
                if command_args.len() >= 2 {
                    let term = command_args[1..].join(" ");
                    search(&buffer.contents, &term);
                } else {
                    println!("search for what?");
                }
            },
            "~show" => {
                let mut start_point = 1;
                let mut end_point = buffer.contents.len();

                if command_args.len() >= 2 {
                    if let Ok(start_from_arg) = command_args[1].parse::<usize>() {
                        start_point = start_from_arg;
                    }
                }

                if command_args.len() >= 3 {
                    if let Ok(end_from_arg) = command_args[2].parse::<usize>() {
                        end_point = end_from_arg;
                    }
                }

                if command_args.len() == 2 {
                    if let Ok(start_from_arg) = command_args[1].parse::<usize>() {
                        end_point = start_from_arg;
                    }
                }

                show(&buffer.contents, start_point, end_point);
            },
            
            // Miscellaneous commands
            "~bsod" => { crash("USER_IS_STUPID", &[0x0000DEAD, 0x00000101, 0xFFFFFFFF, 56]); },
            "~run"  => { shell_command(command_args.clone()); },
            "~runhere" => { 
                let command_args_string = command_args.iter().map(|&s| s.to_string()).collect();
                shell_command_with_file(command_args_string, &mut buffer.contents); 
            }
            "~nothing" => { nothing(&buffer.contents); },

            // Exit command
            "~exit" => break,

            // Fallback
            _       => { 
                if command_args[0].starts_with('~') {
                    println!("{} is an unknown command", command_args[0]);
                }
                else {
                    let to_write = command.clone();
                    buffer.contents.push(to_write);
                }
            }
        };
    }
}
