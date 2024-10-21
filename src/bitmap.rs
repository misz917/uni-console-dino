pub use crate::utils::XY;

pub struct Bitmap<T> {
    pub resolution: XY<usize>,
    pub map: Vec<Vec<T>>,
}

impl<T: Clone> Bitmap<T> {
    pub fn new(resolution: XY<usize>, default_contents: T) -> Self {
        Bitmap {
            resolution: resolution,
            map: vec![vec![default_contents.clone(); resolution.x]; resolution.y],
        }
    }
}

pub struct BitmapRenderer;
impl BitmapRenderer {
    pub fn print_bitmap(bitmap: &Bitmap<char>) {
        for y in 0..bitmap.resolution.y {
            for x in 0..bitmap.resolution.x {
                print!("{}", bitmap.map[y][x]);
            }
            if y + 1 != bitmap.resolution.y {
                print!("\n");
            }
        }
    }
}

pub struct BitmapBuffer {
    pub active_frame: Bitmap<char>,
    pub following_frame: Bitmap<char>,
    // eventually add bool bitmap for changes
}