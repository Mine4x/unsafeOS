use alloc::{format, string::ToString};
use crate::{fs, print};
use super::PWD;

pub static CMD: &str = "ls";
pub static USAGE: &str = "ls [path]";
pub static DES: &str = "Lists directory contents";

pub fn main(args: &[&str]) {
    // Use PWD if no path provided
    let path = if args.is_empty() {
        PWD
    } else {
        args[0]
    };
    
    // Resolve to full path if relative
    let full_path = if path.starts_with('/') {
        path.to_string()
    } else {
        if PWD == "/" {
            format!("/{}", path)
        } else {
            format!("{}/{}", PWD, path)
        }
    };
    
    match fs::list_dir(&full_path) {
        Ok(contents) => {
            if contents.is_empty() {
                print!("\n(empty directory)");
            } else {
                for item in contents {
                    print!("\n{}", item);
                }
            }
        }
        Err(e) => {
            print!("\nError listing '{}': {}", full_path, e);
        }
    }
}