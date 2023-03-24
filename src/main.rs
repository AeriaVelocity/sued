// main.rs - sued, the text editor that doesn't give a damn, short for Shut Up Editor

use std::io;
use std::fs;
use std::path;
use std::process::Command;
use which::which;
use rand::Rng;

fn startup_message() {
    let messages: Vec<&str> = vec![
        "the editor of all time",
        "shut up and edit",
        "the nonstandard text editor",
        "sued as in editor, not as in law",
        "sued, man! ~run man sued",
        "there is no visual mode",
        "the itor fell off",
        "the editor that edits itself",
        "no distractions, just text",
        "the ultimate blank slate",
        "words matter; nothing else does",
        "the text editor that doesn't care",
        "write like no one is watching, because they're not",
        "a hacker's weapon of choice",
    ];
    let message: &str = messages[rand::thread_rng().gen_range(0..messages.len())];
    println!("sued - {message}\ntype ~ for commands, otherwise just start typing");
}

fn display_help() {
    println!("~save, ~open, ~show, ~run, ~exit, ~help");
}

fn extended_help() {
    println!("sued is a line editor, like ed but also not at all\n\
              to write stuff, just start typing after the welcome message\n\
              editor commands are prefixed with ~ (for example ~exit to quit the editor)\n\
              there's no regex stuff or syntax highlighting or anything like that. you just write\n\
              sued written by Arsalan Kazmi (That1M8Head)");
}

fn save(buffer_contents: Vec<String>, file_path: &str) {
    println!("sorry, can't save yet"); // TODO the file buffer exists now, so implement saving!!
}

fn show(buffer_contents: Vec<String>) {
    if buffer_contents.len() < 1 {
        println!("no buffer contents");
    }
    else {
        for line in buffer_contents.iter() {
            println!("{}", line);
        }
    }
}

fn open(command_args: Vec<&str>) -> String {
    if command_args.len() <= 1 {
        println!("open what?");
        return "".to_string();
    }
    else {
        let file_exists = fs::read_to_string(command_args[1]);
        match file_exists {
            Ok(_) => {},
            Err(_) => println!("file {} doesn't exist, so it will be created", command_args[1])
        }
        return fs::read_to_string(command_args[1]).unwrap();
    }
}

fn shell_command(mut command_args: Vec<&str>) {
    if command_args.len() <= 1 {
        println!("run what?");
    }
    else {
        let cmdpath = which(command_args[1]);
        let command = command_args[1];
        match which(command) {
            Ok(_) => println!("running {}", command),
            Err(_) => {
                println!("no such command");
                return;
            }
        }
        command_args.drain(0..2);
        Command::new(cmdpath.clone().unwrap())
            .args(command_args.clone())
            .status()
            .expect("broken!");
        println!("finished running {}", command);
    }
}

fn main() {
    startup_message();
    let mut command: String = String::new();
    let mut file_buffer: Vec<String> = Vec::new();
    let mut file_path: String = String::new();
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
            "~save" => { save(file_buffer.clone(), "nowhere"); },
            "~show" => { show(file_buffer.clone()); },
            "~open" => { 
                file_path = open(command_args.clone());
                if file_path != "" {
                    println!("file {} opened", command_args[1]);
                }
            },
            "~run"  => { shell_command(command_args); },
            "~exit" => { /* do nothing, because `~exit` will break the loop */},
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
