use std::env;

pub mod window;
pub mod utils;
pub mod bitmap;

pub use crate::{utils::XY, bitmap::{Bitmap, BitmapRenderer}};

const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 45);
//160x90 but x axis is 2 times denser

struct BitmapBuffer {
    active_frame: Bitmap<char>,
    following_frame: Bitmap<char>,
}

struct AssetServer;

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
