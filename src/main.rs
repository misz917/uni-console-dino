use std::env;

pub mod asset_server;
pub mod bitmap;
pub mod utils;
pub mod window;

use window::{GnomeTerminal, Terminal};

pub use crate::{
    bitmap::{Bitmap, BitmapRenderer},
    utils::{ANSI, XY},
};

const WINDOW_BORDER_WIDTH: usize = 1;
const WINDOW_RESOLUTION: XY<usize> = XY::new(180, 50);
// const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 45);

const RESET: &str = "\x1B[0m";
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const BLUE: &str = "\x1B[34m";

fn prepare() {
    print!("{}[H", ANSI); // move to 0,0
    print!("{}[1m", ANSI); // enable bold mode
    print!("{}[48;2;{};{};{}m", ANSI, 255, 0, 100); // set background color rgb
    print!("{}[38;2;{};{};{}m", ANSI, 255, 255, 255); // set foreground color rgb
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
        window::WindowCreator::open_new_window(window::GnomeTerminal, WINDOW_RESOLUTION);
        return;
    }

    // pre-startup
    prepare();
    let bitmap = Bitmap::new(WINDOW_RESOLUTION, '#');
    BitmapRenderer::print_bitmap(&bitmap, 1);

    // main loop
    loop {}
}
