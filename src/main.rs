// main.rs - sued, the text editor that doesn't give a damn, short for Shut Up Editor

use std::io;
use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use which::which;
use rand::Rng;

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

fn display_help() {
    println!("~save, ~open, ~show, ~replace, ~delete, ~run, ~exit, ~help");
}

fn extended_help() {
    let version = env!("CARGO_PKG_VERSION");
    println!("this is sued, v{version}\n\
              sued is a line editor, like ed but also not at all\n\
              to write stuff, just start typing after the welcome message\n\
              editor commands are prefixed with ~ (for example ~exit to quit the editor)\n\
              there's no regex stuff or syntax highlighting or anything like that. you just write\n\
              sued written by Arsalan Kazmi (That1M8Head)");
}

fn save(buffer_contents: Vec<String>, file_path: &str) {
    if buffer_contents.is_empty() {
        println!("buffer empty - nothing to save");
        return;
    }

    let content = buffer_contents.join("\n");
    println!("{}", &file_path);
    let path = PathBuf::from(file_path);

    match fs::write(&path, content) {
        Ok(_) => println!("saved to {}", &path.display()),
        Err(error) => eprintln!("couldn't save file: {}", error),
    }
}

fn show(buffer_contents: Vec<String>) {
    if buffer_contents.len() < 1 {
        println!("no buffer contents");
    }
    else {
        let mut count: i32 = 0;
        for line in buffer_contents.iter() {
            count += 1;
            println!("{}| {}", count, line);
        }
    }
}

fn open(file_path: &str) -> Vec<String> {
    let file_exists = fs::read_to_string(file_path);
    match file_exists {
        Ok(contents) => {
            println!("file {} opened", file_path);
            return contents.lines().map(|line| line.to_owned()).collect();
        }
        Err(_) => {
            println!("no such file {}", file_path);
            return Vec::new();
        }
    }
}

fn replace(file_buffer: &mut Vec<String>, line_number: usize) {
    if line_number <= file_buffer.len() {
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
    } else {
        println!("no line {}", line_number);
    }
}

fn delete(file_buffer: &mut Vec<String>, line_number: usize) {
    if line_number <= file_buffer.len() {
        file_buffer.remove(line_number - 1);
    } else {
        println!("no line {}", line_number);
    }
}

fn shell_command(mut command_args: Vec<&str>) {
    if command_args.len() <= 1 {
        println!("run what?");
    } else {
        let command = command_args[1];
        let shell = if cfg!(windows) { 
            "cmd" 
        } else { 
            "sh" 
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
            .arg("/c")
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
                crash("editor_overflow", &vec![0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF])
            },
            _ => ()
        }
    }
}

fn main() {
    startup_message();
    let mut command: String = String::new();
    let mut file_buffer: Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        file_buffer = open(&args[1]);
    }
    while !command.eq("~exit") {
        command.clear();
        io::stdin()
            .read_line(&mut command)
            .expect("can't read command");
        let len: usize = command.trim_end_matches(&['\r', '\n'][..]).len();
        command.truncate(len);
        let command_args = command.split(" ").collect::<Vec<&str>>();
        let _cmdproc: () = match command_args[0] {
            "~"     => { display_help(); },
            "~help" => { extended_help(); },
            "~save" => {
                if command_args.len() >= 2 {
                    save(file_buffer.clone(), command_args[1]);
                }
                else {
                    println!("save what?");
                }
            },
            "~show" => { show(file_buffer.clone()); },
            "~open" => { 
                if command_args.len() >= 2 {
                    file_buffer = open(command_args[1]);
                }
                else {
                    println!("open what?");
                }
            },
            "~run"  => { shell_command(command_args); },
            "~bsod" => { crash("USER_IS_STUPID", &vec![0x0000DEAD, 0x00000101, 0xFFFFFFFF, 56]); },
            "~replace" => {
                if command_args.len() >= 2 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    replace(&mut file_buffer, line_number);
                } else {
                    println!("replace which line?");
                }
            },
            "~delete" => {
                if command_args.len() >= 2 {
                    let line_number = command_args[1].parse::<usize>().unwrap_or(0);
                    delete(&mut file_buffer, line_number);
                } else {
                    println!("delete which line?");
                }
            }
            "~exit" => { /* do nothing, because `~exit` will break the loop */ },
            _       => { 
                if command_args[0].starts_with("~") {
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
