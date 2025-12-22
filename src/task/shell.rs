use alloc::string::{String, ToString};
use pc_keyboard::{DecodedKey};
use spin::Mutex;
use crate::{print, vga_buffer};
use super::keyboard;
pub mod hello;
pub mod clear;
pub mod cat;
pub mod mkdir;
pub mod touch;
pub mod ls;
pub mod test;

// NOTE: Each command module must have this signature:
// pub const CMD: &str = "command_name";
//pub static USAGE: &str = "command_usage"; (only for help command)
//pub static DES: &str = "command_description"; (only for help command)
// pub fn main(args: &[&str]) { ... }

static TYPED_KEYS: Mutex<String> = Mutex::new(String::new());

/////TODO: somehow allow cd to change PWD without causing other errors
pub static PWD: &str = "/";

// Define command structure
struct Command {
    name: &'static str,
    usage: &'static str,
    des: &'static str,
    handler: fn(&[&str]),
}

// List of all available commands
const COMMANDS: &[Command] = &[
    Command { name: hello::CMD, handler: hello::main, usage: hello::USAGE, des: hello::DES },
    Command { name: clear::CMD, handler: clear::main, usage: clear::USAGE, des: clear::DES },
    Command { name: cat::CMD, handler: cat::main, usage: cat::USAGE, des: cat::DES },
    Command { name: mkdir::CMD, handler: mkdir::main, usage: mkdir::USAGE, des: mkdir::DES},
    Command { name: touch::CMD, handler: touch::main, usage: touch::USAGE, des: touch::DES},
    Command { name: ls::CMD, handler: ls::main, usage: ls::USAGE, des: ls::DES},
    Command { name: test::CMD, handler: test::main, usage: test::USAGE, des: test::DES}
];

fn handle_cmd() {
    let binding = TYPED_KEYS.lock();
    let input = binding.as_str().trim();
    
    // Split input into command and arguments
    let parts: alloc::vec::Vec<&str> = input.split_whitespace().collect();
    
    if parts.is_empty() {
        return;
    }
    
    let cmd = parts[0];
    let args = &parts[1..];
    
    if cmd == "help" {
        print!("\n\n-- help list --\n\n");
        for command in COMMANDS {
            print!("-- {} --\n", command.name);
            print!("USAGE: {}\n", command.usage);
            print!("DESCRIPTION: {}\n", command.des);
        }
        return;
    }

    // Loop through all commands to find a match
    for command in COMMANDS {
        if cmd == command.name {
            (command.handler)(args);
            return;
        }
    }
    
    // If no command matched, optionally print an error
    print!("\nUnknown command: {}", cmd);
}

fn handle_unicode(c: char) {
    let key = c.to_string();
    if key == "\n" {
        handle_cmd();
        print!("\n$ ");
        vga_buffer::WRITER.lock().set_column_position(2);
        
        let mut ____s____ = TYPED_KEYS.lock();
        ____s____.clear();
    }
    else if key == "\x08" || key == "\x7f" {
        let binding = TYPED_KEYS.lock();
        let length = binding.to_string().len();
        drop(binding);
        if length > 0 {
            let mut ____s____ = TYPED_KEYS.lock();
            ____s____.pop();
            vga_buffer::WRITER.lock().backspace();
        }
    }
    else {
        let mut ____s____ = TYPED_KEYS.lock();
        ____s____.push(c);
        print!("{}", c);
    }
}

fn key_pressed(key: DecodedKey) {
    match key {
        DecodedKey::Unicode(c) => {handle_unicode(c);},
        _ => {}
    }
}

pub async fn init() {
    vga_buffer::WRITER.lock().clear_buffer(); // So the cursor gets shown even before using clean
    keyboard::register_key_callback(key_pressed);
    print!("$ ");
}