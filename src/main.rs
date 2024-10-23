use std::{
    env,
    process::exit,
    thread::sleep,
    time::{Duration, SystemTime},
};

pub mod asset_server;
pub mod bitmap;
pub mod screen;
pub mod utils;
pub mod window;

use crate::{
    bitmap::{Bitmap, BitmapBuffer},
    utils::{ANSI, XY},
    screen::TerminalScreen,
};

const BORDER_WIDTH: XY<usize> = XY::new(2, 2);
const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 40);
const FPS_LIMIT: f32 = 30.0;

fn prepare() {
    print!("{}[H", ANSI); // move to 0,0
    print!("{}[1m", ANSI); // enable bold mode
    print!("{}[48;2;{};{};{}m", ANSI, 255, 0, 100); // set background color rgb
    print!("{}[38;2;{};{};{}m", ANSI, 127, 127, 127); // set foreground color rgb
    print!("{}[?25l", ANSI); // make cursor invisible
}

fn separate_window_creation() {
    let args: Vec<String> = env::args().collect();
    let mut ready: bool = false;
    for arg in &args {
        if arg == "-ready" {
            ready = true;
        }
    }
    if !ready {
        window::WindowCreator::open_new_window(
            window::GnomeTerminal,
            WINDOW_RESOLUTION,
            BORDER_WIDTH,
        );
        exit(0);
    }
}

fn main() {
    separate_window_creation();

    let mut screen = TerminalScreen::new_default(WINDOW_RESOLUTION, BORDER_WIDTH);

    let sleep_duration = 1.0 / FPS_LIMIT;
    loop {
        let time = SystemTime::now();

        screen.display();

        if let Ok(elapsed) = time.elapsed() {
            sleep(Duration::from_secs_f32(sleep_duration) - elapsed);
        } else {
            return;
        }
    }
}
