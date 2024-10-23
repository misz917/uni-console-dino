use crate::{
    bitmap::{Bitmap, BitmapBuffer, BitmapRenderer},
    utils::{ANSI, XY},
};
use std::io::{self, Write};

pub struct TerminalScreen {
    bitmap_buffer: BitmapBuffer,
    border_width: XY<usize>,
}
impl TerminalScreen {
    pub fn new(bitmap_buffer: BitmapBuffer, border_width: XY<usize>) -> Self {
        TerminalScreen {
            bitmap_buffer,
            border_width,
        }
    }
    pub fn print_screen(&self) {
        Self::clear_screen();
        BitmapRenderer::print_bitmap(&self.bitmap_buffer.active_frame, &self.border_width);
        Self::flush_terminal_buffer();
    }

    fn clear_screen() {
        print!("{}[J", ANSI);
    }

    // fixes a problem that shouldn't exist
    fn flush_terminal_buffer() {
        io::stdout().flush().unwrap();
    }
}
