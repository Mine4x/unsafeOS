use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use crate::print;
use crate::fs::read;
use super::PWD;

pub static CMD: &str = "cat";
pub static USAGE: &str = "cat (path)";
pub static DES: &str = "displays the content of a file";

fn read_file(path: &str) -> Result<alloc::string::String, alloc::string::String> {
    let data = read(path)?;
    alloc::string::String::from_utf8(data)
        .map_err(|e| format!("Invalid UTF-8: {}", e))
}

pub fn main(args: &[&str]) {
    if args.is_empty() || args.len() > 1 {
        print!("\nUSAGE: cat (path)");
    } else {
        let chars: Vec<char> = args[0].chars().collect();
        let path = if chars[0] == '/' {
            args[0].to_string()
        } else {
            format!("{}/{}", PWD, args[0])
        };

        match read_file(&path) {
            Ok(content) => print!("\n{}", content),
            Err(e) => print!("\ncat: {}", e),
        }
    }
}