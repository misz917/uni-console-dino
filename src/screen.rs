use crate::{
    bitmap::{BitmapBuffer, BitmapRenderer},
    utils::{ANSI, XY},
};
use std::io::{self, Write};

pub struct TerminalScreen {
    bitmap_buffer: BitmapBuffer,
    border_width: XY<usize>,
}
impl TerminalScreen {
    pub fn new(bitmap_buffer: &BitmapBuffer, border_width: XY<usize>) -> Self {
        TerminalScreen {
            bitmap_buffer: bitmap_buffer.clone(),
            border_width,
        }
    }
    pub fn display(&mut self) {
        Self::move_cursor_home();
        let bitmap_to_display = self.bitmap_buffer.get_active_frame();
        BitmapRenderer::print_bitmap(bitmap_to_display, &self.border_width);
        Self::flush_terminal_buffer();
        // self.bitmap_buffer.update(); // should be done while adding a new frame
    }

    // fixes a problem where terminal doesn't print out a number of characters
    fn flush_terminal_buffer() {
        io::stdout().flush().unwrap();
    }

    fn move_cursor_home() {
        print!("{}[H", ANSI);
    }
}
