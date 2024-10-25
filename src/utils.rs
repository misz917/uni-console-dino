use std::{ops::Add, thread::sleep, time::Duration};
use crate::terminal_screen::TerminalScreen;

pub const ESC: &str = "\x1B";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

impl<T> XY<T> {
    pub const fn new(x: T, y: T) -> Self {
        XY { x, y }
    }
}

impl<T> XY<T>
where
    T: Add<Output = T> + Copy,
{
    pub fn add(&mut self, val: &XY<T>) {
        self.x = self.x + val.x;
        self.y = self.y + val.y;
    }
}

pub struct ErrorDisplayer;
impl ErrorDisplayer {
    pub fn error(message: &str) {
        print!("{}[H{}", ESC, message);
        TerminalScreen::flush_terminal_buffer();
        loop {
            sleep(Duration::from_secs(1));
        }
    }
}