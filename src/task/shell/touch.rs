use alloc::{format, string::{String, ToString}};
use crate::{fs, print};
use super::PWD;

pub static CMD: &str = "touch";
pub static USAGE: &str = "touch <path> [\"content\"]";
pub static DES: &str = "creates a new file or updates timestamp, optionally with content";

pub fn main(args: &[&str]) {
    if args.is_empty() {
        print!("\nUSAGE: {}\n", USAGE);
        return;
    }
    
    // Parse arguments: first is path, rest might be content in quotes
    let path = args[0];
    let content = if args.len() > 1 {
        parse_content(&args[1..])
    } else {
        String::new()
    };
    
    if let Err(e) = create_file(path, content.as_bytes()) {
        print!("Error creating '{}': {}\n", path, e);
    }
}

fn parse_content(args: &[&str]) -> String {
    // Join all remaining arguments and handle quotes
    let joined = args.join(" ");
    
    // Remove surrounding quotes if present
    if (joined.starts_with('"') && joined.ends_with('"')) 
        || (joined.starts_with('\'') && joined.ends_with('\'')) {
        if joined.len() >= 2 {
            return joined[1..joined.len()-1].to_string();
        }
    }
    
    joined
}

fn create_file(path: &str, content: &[u8]) -> Result<(), String> {
    if path.is_empty() {
        return Err("path cannot be empty".to_string());
    }
    
    let is_absolute = path.starts_with('/');
    
    // Build the full path
    let full_path = if is_absolute {
        path.to_string()
    } else {
        if PWD == "/" {
            format!("/{}", path)
        } else {
            format!("{}/{}", PWD, path)
        }
    };
    
    // Write the file (creates it if it doesn't exist)
    fs::write(&full_path, content)
}

// Alternative version if you want to create multiple files at once
pub fn main_multi(args: &[&str]) {
    if args.is_empty() {
        print!("\nUSAGE: {}\n", USAGE);
        return;
    }
    
    // Find where content starts (look for quoted string)
    let mut content_start = None;
    for (i, arg) in args.iter().enumerate() {
        if arg.starts_with('"') || arg.starts_with('\'') {
            content_start = Some(i);
            break;
        }
    }
    
    let (paths, content) = if let Some(idx) = content_start {
        let paths = &args[..idx];
        let content = parse_content(&args[idx..]);
        (paths, content)
    } else {
        (args, String::new())
    };
    
    // Create each file with the same content
    for path in paths {
        if let Err(e) = create_file(path, content.as_bytes()) {
            print!("Error creating '{}': {}\n", path, e);
        }
    }
}