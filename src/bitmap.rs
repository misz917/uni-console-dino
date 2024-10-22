
use std::io::{self, Write};

pub use crate::utils::{ANSI, XY};

pub struct Bitmap<T> {
    pub resolution: XY<usize>,
    pub map: Vec<Vec<T>>,
}

impl<T: Clone> Bitmap<T> {
    pub fn new(resolution: XY<usize>, default_contents: T) -> Self {
        Bitmap {
            resolution,
            map: vec![vec![default_contents.clone(); resolution.x]; resolution.y],
        }
    }
}

pub struct BitmapRenderer;
impl BitmapRenderer {
    pub fn print_bitmap(bitmap: &Bitmap<char>, border_width: usize) {
        for y in 0..bitmap.resolution.y {
            // print!("{}[{};{}f", ANSI, 2 + y, 2);
            for x in 0..bitmap.resolution.x {
                print!("{}[{};{}f{}", ANSI, y, x, bitmap.map[y][x]);
            }
        }
        Self::flush_terminal_buffer();
    }

    // fixes a problem that shouldn't exist
    fn flush_terminal_buffer() {
        io::stdout().flush().unwrap();
    }
}

pub struct BitmapBuffer {
    pub active_frame: Bitmap<char>,
    pub following_frame: Bitmap<char>,
    // eventually add bool bitmap for changes
}
