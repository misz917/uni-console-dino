use std::process::Command;

#[derive(Clone, Copy, Debug)]
struct XY<T> {
    x: T,
    y: T,
}

impl<T> XY<T> {
    const fn new(x: T, y: T) -> Self {
        XY { x, y }
    }
}

const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 45); //160x90 but x axis is 2 times denser

struct Bitmap<T> {
    resolution: XY<usize>,
    map: Vec<Vec<T>>,
}

impl<T: Clone> Bitmap<T> {
    fn new(resolution: XY<usize>, default_contents: T) -> Self {
        Bitmap {
            resolution: resolution,
            map: vec![vec![default_contents.clone(); resolution.x]; resolution.y],
        }
    }
}

struct BitmapBuffer {
    active_frame: Bitmap<char>,
    following_frame: Bitmap<char>,
}

struct AssetServer;

struct BitmapRenderer;

impl BitmapRenderer {
    fn print_bitmap(bitmap: &Bitmap<char>) {
        
    }
}

fn main() {
    println!("Hello world!");
}
