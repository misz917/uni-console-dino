use std::ops::Add;

use crate::bitmap::Bitmap;

pub const ANSI: &str = "\x1B";

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

pub struct Sprite {
    pub dimensions: XY<usize>,
    pub bitmap: Bitmap<char>,
}
