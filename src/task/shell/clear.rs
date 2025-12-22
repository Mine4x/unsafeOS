use crate::vga_buffer;

pub static CMD: &str = "clear";
pub static USAGE: &str = "clear";
pub static DES: &str = "Clears the shell";

pub fn main(_args: &[&str]) {
    vga_buffer::WRITER.lock().clear_buffer();    
}