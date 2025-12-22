use vga::colors::Color16;
use crate::vga_buffer::WRITER;

pub fn play() {
    WRITER.lock().switch_to_graphics();

    if let Some(gfx) = WRITER.lock().get_graphics() {
        gfx.clear_screen(Color16::Black);
        gfx.draw_line((80, 60), (540, 0), Color16::White);
        gfx.draw_line((80, 60), (540, 60), Color16::LightRed);
        gfx.draw_line((540, 0), (540, 60), Color16::Yellow);
    }

    for _ in 0..10_000_000 {
        core::hint::spin_loop();
    }

    WRITER.lock().switch_to_text();
}