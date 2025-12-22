use crate::print;

pub static CMD: &str = "test";
pub static USAGE: &str = "test [test]";
pub static DES: &str = "tests some features like: VGA graphics mode";

mod anim;
mod graphics;

pub fn main(args: &[&str]) {
    if args.is_empty() {
        print!("Usage: {}", USAGE);
        return;
    }

    if args[0] == "anim" {
        anim::play();
    } else if args[0] == "graphics" {
        graphics::play();
    }
}