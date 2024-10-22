use crate::utils::{ANSI, XY};
use std::io::{self, Write};

pub struct TerminalScreen;
impl TerminalScreen {
    // fixes a problem that shouldn't exist
    pub fn flush_buffer() {
        io::stdout().flush().unwrap();
    }

    pub fn clear_screen() {
        print!("{}[J", ANSI);
    }
}
