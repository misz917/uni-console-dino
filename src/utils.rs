use crate::terminal_screen::TerminalHelper;
use std::{
    ops::Add,
    thread::sleep,
    time::{Duration, Instant},
};

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
        TerminalHelper::flush_terminal_buffer();
        loop {
            sleep(Duration::from_secs(1));
        }
    }
}

pub enum Value {
    I32(i32),
    F32(f32),
    Bool(bool),
    Str(String),
    Duration(Duration),
    Instant(Instant),
}
