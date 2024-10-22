use std::env;

pub mod asset_server;
pub mod bitmap;
pub mod utils;
pub mod window;

pub use crate::{
    bitmap::{Bitmap, BitmapRenderer},
    utils::{ANSI, XY},
};

const WINDOW_BORDER_WIDTH: usize = 1;
const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 40);

fn prepare() {
    print!("{}[H", ANSI); // move to 0,0
    print!("{}[1m", ANSI); // enable bold mode
    print!("{}[48;2;{};{};{}m", ANSI, 255, 0, 100); // set background color rgb
    print!("{}[38;2;{};{};{}m", ANSI, 127, 127, 127); // set foreground color rgb
    print!("{}[?25l", ANSI); // make cursor invisible
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
        window::WindowCreator::open_new_window(window::GnomeTerminal, WINDOW_RESOLUTION, WINDOW_BORDER_WIDTH);
        return;
    }

    // ----Startup----
    // prepare();
    let bitmap = Bitmap::new(WINDOW_RESOLUTION, '#');
    BitmapRenderer::print_bitmap(&bitmap, 1);

    // ----Main loop----
    loop {
        
    }
}
