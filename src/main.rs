use std::env;
pub mod window;
pub mod utils;
pub use crate::utils::XY;

const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 45);
//160x90 but x axis is 2 times denser

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ready: bool = false;
    for arg in &args {
        if arg == "-ready" {
            ready = true;
        }
    }
    
    if !ready {
        window::WindowCreator::open_new_window(window::GnomeTerminal, WINDOW_RESOLUTION);
        return;
    }

    // pre-startup
    let bitmap = Bitmap::new(WINDOW_RESOLUTION, '#');
    BitmapRenderer::print_bitmap(&bitmap);

    // main loop
    loop {
        
    }
}
