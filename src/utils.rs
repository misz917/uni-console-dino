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