use alloc::{format, string::{String, ToString}, vec::Vec};
use crate::{fs, print};
use super::PWD;

pub static CMD: &str = "mkdir";
pub static USAGE: &str = "mkdir <path>";
pub static DES: &str = "creates one or multiple new directories";

pub fn main(args: &[&str]) {
    if args.is_empty() {
        print!("\nUSAGE: {}\n", USAGE);
        return;
    }
    
    for arg in args {
        if let Err(e) = create_directory(arg) {
            print!("Error creating '{}': {}\n", arg, e);
        }
    }
}

fn create_directory(path: &str) -> Result<(), &'static str> {
    if path.is_empty() {
        return Err("path cannot be empty");
    }
    
    let is_absolute = path.starts_with('/');
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    
    if parts.is_empty() {
        return Err("invalid path");
    }
    
    // Build the full path
    let mut current_dir = if is_absolute {
        String::from("")
    } else {
        PWD.to_string()
    };
    
    // Create each directory in the path
    for part in parts {
        current_dir = if current_dir.is_empty() || current_dir == "/" {
            format!("/{}", part)
        } else {
            format!("{}/{}", current_dir, part)
        };
        
        // Attempt to create the directory
        if let Err(_) = fs::create_dir(&current_dir) {
            // Directory might already exist, which is typically fine
            // or there was an actual error - depends on your fs implementation
        }
    }
    
    Ok(())
}