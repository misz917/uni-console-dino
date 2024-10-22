use std::io::{self, Write};
use crate::utils::{ANSI, XY};

pub struct TerminalScreen;
impl TerminalScreen {
        // fixes a problem that shouldn't exist
        fn flush_buffer() {
            io::stdout().flush().unwrap();
        }

        fn clear_screen() {
            print!("{}[J", ANSI);
        }
}