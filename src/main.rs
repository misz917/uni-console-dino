pub struct XY<T> {
    pub x: T,
    pub y: T,
}

impl<T> XY<T> {
    pub const fn new(x: T, y: T) -> Self {
        XY { x, y }
    }
}

pub const WINDOW_RESOLUTION: XY<usize> = XY::new(80, 45);

pub struct ScreenBuffer {
    pub active_frame: Vec<Vec<char>>,
    pub following_frame: Vec<Vec<char>>,
}

pub struct SpriteDrawer;

impl SpriteDrawer {}

fn main() {
    println!("Hello world!");
}
