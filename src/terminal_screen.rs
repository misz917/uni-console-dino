use std::io::{self, Write};
use crate::{
    bitmap::{Bitmap, BitmapPrinter},
    bitmap_buffer::BitmapBuffer,
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