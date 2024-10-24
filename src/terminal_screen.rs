use crate::{
    bitmap::{Bitmap, BitmapBuffer, BitmapPrinter},
    utils::{ESC, XY},
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

    pub fn new_default(resolution: XY<usize>, border_width: XY<usize>) -> Self {
        let bitmap = Bitmap::new(resolution, '#');
        TerminalScreen {
            bitmap_buffer: BitmapBuffer::new(&bitmap),
            border_width,
        }
    }

    pub fn schedule_frame(&mut self, new_frame: &Bitmap<char>) {
        self.bitmap_buffer.new_following_frame(new_frame);
    }

    pub fn display_frame(&mut self) {
        Self::move_cursor_home();
        BitmapPrinter::print_bitmap(&self.bitmap_buffer.get_active_frame(), &self.border_width);
        self.bitmap_buffer.update();
        Self::flush_terminal_buffer();
    }

    // fixes a problem where terminal doesn't print out a number of characters
    pub fn flush_terminal_buffer() {
        io::stdout().flush().unwrap();
    }

    fn move_cursor_home() {
        print!("{}[H", ESC);
    }

    pub fn prepare() {
        print!("{}[H", ESC); // move to 0,0
        print!("{}[1m", ESC); // enable bold mode
        print!("{}[48;2;{};{};{}m", ESC, 255, 0, 100); // set background color rgb
        print!("{}[38;2;{};{};{}m", ESC, 127, 127, 127); // set foreground color rgb
        print!("{}[?25l", ESC); // make cursor invisible
    }
}
