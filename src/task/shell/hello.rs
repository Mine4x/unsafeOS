use crate::println;

pub static CMD: &str = "hello";
pub static USAGE: &str = "hello [name]";
pub static DES: &str = "displays a hello message for testing";

pub fn main(args: &[&str]) {
    if args.is_empty() {
        println!("\nHi :)");
    } else {
        println!("\nHi {} :)", args[0]);
    }
}