use crate::{print, vga_buffer, vga_buffer::Color};

pub fn play() {
    vga_buffer::WRITER.lock().clear_buffer();
    //vga_buffer::WRITER.lock().set_color(Color, Color::LightCyan);
    
    for h in 1..10 {
        print!("###############\n");
        print!(" #  #  #  #  #\n");
        
        for _ in 0..10_000_00 {
            core::hint::spin_loop();
        }
    }
    
    //vga_buffer::WRITER.lock().set_color(Color::White, Color::LightCyan);
    print!("###############\n");
    //vga_buffer::WRITER.lock().set_color(Color::White, Color::Black);
}
