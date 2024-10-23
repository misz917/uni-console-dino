use std::{
    env, process::exit, thread::sleep, time::{Duration, SystemTime}
};

pub mod asset_server;
pub mod bitmap;
pub mod frame_assembler;
pub mod terminal_screen;
pub mod utils;
pub mod window;

use bitmap::BitmapRenderer;

use crate::{
    terminal_screen::TerminalScreen,
    utils::XY,
};

const BORDER_WIDTH: XY<usize> = XY::new(2, 2); // buggy
const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 40);
const FPS_LIMIT: f32 = 30.0; // buggy above ~46

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
        exit(0); // this exit is not an error
    }
}

fn main() {
    separate_window_creation();
    let sleep_duration = 1.0 / FPS_LIMIT;

    let mut screen = TerminalScreen::new_default(WINDOW_RESOLUTION, BORDER_WIDTH);
    let path = "/home/firstuser/Codes/githubRepos/uni-console-dino/src/assets/dino_sprite.txt";
    let sprite = crate::asset_server::SpriteFileReader::read(&path);
    // BitmapRenderer::print_bitmap(&sprite, &BORDER_WIDTH);
    

    loop {
        let time = SystemTime::now();

        // screen.display_frame();

        if let Ok(elapsed) = time.elapsed() {
            sleep(Duration::from_secs_f32(sleep_duration) - elapsed);
        } else {
            return;
        }
    }
}
