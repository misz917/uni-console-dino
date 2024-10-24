use std::{
    env,
    thread::sleep,
    time::{Duration, SystemTime},
};

pub mod asset_server;
pub mod bitmap;
pub mod frame_assembler;
pub mod terminal_screen;
pub mod utils;
pub mod window;

use bitmap::Bitmap;
use frame_assembler::FrameAssembler;
use utils::Sprite;
use window::{Terminal, UnixTerminalHandler};

use crate::{
    terminal_screen::TerminalScreen,
    utils::XY,
    window::{GnomeTerminal, WindowCreator},
};

// create a settings file later
const BORDER_WIDTH: XY<usize> = XY::new(2, 1);
const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 40);
const FPS_LIMIT: f32 = 5.0; // buggy above ~46

fn debug_sprite_load(sprite_name: &str) -> Sprite {
    let binding = env::current_exe().unwrap();
    let binding = binding
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let path = binding.to_string_lossy() + "/src/assets/" + sprite_name;
    let sprite = Sprite::from_bitmap(&crate::asset_server::SpriteFileReader::read(&path));
    return sprite;
}

fn main() {
    let sleep_duration = 1.0 / FPS_LIMIT;
    let gnome = GnomeTerminal::new();
    WindowCreator::create_separate_window(WINDOW_RESOLUTION, BORDER_WIDTH, &gnome);
    gnome.set_raw_mode();

    let mut screen = TerminalScreen::new_default(WINDOW_RESOLUTION, BORDER_WIDTH);
    TerminalScreen::prepare();

    
    let sprite = debug_sprite_load("dino_sprite.txt");

    let mut new_frame = Bitmap::new(WINDOW_RESOLUTION, '#');
    FrameAssembler::write_sprite_to_bitmap(&sprite, &mut new_frame, &XY::new(-1, 35));
    screen.schedule_frame(&new_frame);

    let mut _frame_count: u128 = 0;
    loop {
        let time = SystemTime::now();

        screen.display_frame();

        if let Ok(elapsed) = time.elapsed() {
            sleep(Duration::from_secs_f32(sleep_duration) - elapsed);
        } else {
            return;
        }
        _frame_count += 1;
    }
}
