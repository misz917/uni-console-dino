use std::vec;

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

struct Bitmap<T> {
    resolution: XY<usize>,
    map: Vec<Vec<T>>,
}

impl<T: std::clone::Clone> Bitmap<T> {
    fn new(resolution: XY<usize>, default_contents: T) -> Self {
        Bitmap {
            resolution: resolution.clone(),
            map: vec![vec![default_contents; resolution.x]; resolution.y],
        }
    }
}

pub struct ScreenBuffer {
    pub active_frame: Vec<Vec<char>>,
    pub following_frame: Vec<Vec<char>>,
}

pub struct AssetServer;

pub struct BitmapRenderer;

fn main() {
    println!("Hello world!");
}
