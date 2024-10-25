use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

pub mod asset_server;
pub mod bitmap;
pub mod frame_assembler;
pub mod terminal_screen;
pub mod utils;
pub mod window;
pub mod bitmap_buffer;
pub mod bitmap_wrapper;

use crate::{
    terminal_screen::TerminalScreen,
    utils::XY,
    window::{GnomeTerminal, WindowCreator, Terminal},
    asset_server::AssetServer,
    frame_assembler::FrameAssembler,
    bitmap_wrapper::Label
};

// create a settings file later
const BORDER_WIDTH: XY<usize> = XY::new(2, 1);
const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 40);
const FPS_LIMIT: f32 = 10.0; // buggy above ~46

fn main() {
    let sleep_duration = 1.0 / FPS_LIMIT;
    let gnome = GnomeTerminal::new();
    WindowCreator::create_separate_window(WINDOW_RESOLUTION, BORDER_WIDTH, &gnome);
    gnome.set_raw_mode();

    let mut screen = TerminalScreen::new_default(WINDOW_RESOLUTION, BORDER_WIDTH);
    TerminalScreen::prepare();

    // let mut asset_server = AssetServer::new("/home/firstuser/Codes/githubRepos/uni-console-dino/src/assets/");

    let mut _frame_count: u128 = 0;
    loop {
        let time = SystemTime::now();

        let mut frame_assembler = FrameAssembler::new(WINDOW_RESOLUTION);
        frame_assembler.insert(&Label::new(&_frame_count.to_string()), &XY::new(1, 1));
        screen.schedule_frame(&frame_assembler.get_frame());

        screen.display_frame();

        if let Ok(elapsed) = time.elapsed() {
            sleep(Duration::from_secs_f32(sleep_duration) - elapsed);
        } else {
            return;
        }
        _frame_count += 1;
    }
}
