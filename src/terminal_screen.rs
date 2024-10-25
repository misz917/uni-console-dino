use std::io::{self, Write};
use crate::{
    bitmap::{Bitmap, BitmapPrinter, Printer},
    bitmap_buffer::{BitmapBuffer, BufferManager},
    utils::{ESC, XY},
};

pub struct TerminalHelper;
impl TerminalHelper {
    pub fn prepare_terminal() {
        Self::set_character_color(255, 255, 255);
        Self::set_background_color(10, 50, 150);
        Self::disable_cursor_visibility();
        Self::enable_bold_mode();
        Self::move_cursor_home();
    }

    pub fn flush_terminal_buffer() {
        io::stdout().flush().unwrap();
    }

    fn move_cursor_home() {
        print!("{}[H", ESC);
    }

    fn set_background_color(r: u8, g: u8, b: u8) {
        print!("{}[48;2;{};{};{}m", ESC, r, g, b);
    }

    fn set_character_color(r: u8, g: u8, b: u8) {
        print!("{}[38;2;{};{};{}m", ESC, r, g, b);
    }

    fn disable_cursor_visibility() {
        print!("{}[?25l", ESC);
    }

    fn enable_bold_mode() {
        print!("{}[1m", ESC);
    }
}

pub struct TerminalScreen<B: BufferManager, P: Printer> {
    buffer: B,
    printer: P,
    border_width: XY<usize>,
}
impl<B: BufferManager, P: Printer> TerminalScreen<B, P> {
    pub fn new(buffer: B, printer: P, border_width: XY<usize>) -> Self {
        TerminalScreen {
            buffer,
            printer,
            border_width,
        }
    }

    pub fn schedule_frame(&mut self, new_frame: &Bitmap<char>) {
        self.buffer.new_following_frame(new_frame);
    }

    pub fn display_frame(&mut self) {
        TerminalHelper::move_cursor_home();
        self.printer.print(&self.buffer.get_active_frame(), &self.border_width);
        self.buffer.update();
        TerminalHelper::flush_terminal_buffer();
    }

    pub fn prepare() {
        TerminalHelper::prepare_terminal();
    }
}