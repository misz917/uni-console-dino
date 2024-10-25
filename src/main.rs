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

use asset_server::AssetServer;
use bitmap::Bitmap;
use frame_assembler::FrameAssembler;
use utils::Sprite;
use window::Terminal;

use crate::{
    terminal_screen::TerminalScreen,
    utils::XY,
    window::{GnomeTerminal, WindowCreator},
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


    let mut asset_server = AssetServer::new("/home/firstuser/Codes/githubRepos/uni-console-dino/src/assets/");
    let sprite = asset_server.load("dino_sprite.txt");

    // let mut frame_assembler = FrameAssembler::new(WINDOW_RESOLUTION);
    // frame_assembler.insert_sprite(&sprite, &XY::new(40, 10));
    // let new_frame = *frame_assembler.get_frame();
    // screen.schedule_frame(&new_frame);

    let mut _frame_count: u128 = 0;
    loop {
        let time = SystemTime::now();

        let mut frame_assembler = FrameAssembler::new(WINDOW_RESOLUTION);
        frame_assembler.insert_sprite(&Sprite::from_bitmap(&Bitmap::from_string(&_frame_count.to_string())), &XY::new(1, 1));
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
