use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

pub mod asset_server;
pub mod bitmap;
pub mod bitmap_buffer;
pub mod drawable_object;
pub mod frame_assembler;
pub mod movement_functions;
pub mod terminal_screen;
pub mod utils;
pub mod view;
pub mod window;

use drawable_object::DrawableObject;

use crate::{
    asset_server::AssetServer,
    bitmap::{Bitmap, BitmapPrinter},
    bitmap_buffer::BitmapBuffer,
    drawable_object::Label,
    frame_assembler::FrameAssembler,
    terminal_screen::TerminalHelper,
    terminal_screen::TerminalScreen,
    utils::XY,
    window::{GnomeTerminal, Terminal, WindowCreator},
};

// create a settings file later
const BORDER_WIDTH: XY<usize> = XY::new(2, 1);
const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 40);
const FPS_LIMIT: f32 = 30.0; // buggy above ~46

fn main() {
    let sleep_duration = 1.0 / FPS_LIMIT;
    let gnome_window = GnomeTerminal::new();
    WindowCreator::create_separate_window(WINDOW_RESOLUTION, BORDER_WIDTH, &gnome_window);
    gnome_window.set_raw_mode();

    let bitmap_buffer = BitmapBuffer::new(&Bitmap::new(WINDOW_RESOLUTION, '#'));
    let mut screen = TerminalScreen::new(bitmap_buffer, BitmapPrinter, BORDER_WIDTH);
    TerminalHelper::prepare_terminal();

    // let mut asset_server = AssetServer::new("/home/firstuser/Codes/githubRepos/uni-console-dino/src/assets/");
    let mut asset_server =
        AssetServer::new("/home/user/Codes/githubRepos/uni-console-dino/src/assets/");
    let dino = *asset_server.load("dino.txt");
    let vase = *asset_server.load("vase.txt");
    let mut animation = *asset_server.load("test_animation.txt");

    let mut _frame_count: u128 = 0;
    loop {
        let time = SystemTime::now();
        screen.display_frame();
        let mut frame_assembler = FrameAssembler::new(WINDOW_RESOLUTION);

        let label = DrawableObject::Label(Label::new(&_frame_count.to_string()));
        frame_assembler.insert(&label, &XY::new(1, 1));
        frame_assembler.insert(&dino, &XY::new(3, 33));
        frame_assembler.insert(&vase, &XY::new(30, 34));
        frame_assembler.insert_mut(&mut animation, &XY::new(45, 34));

        // let input = gnome_window.read_key();

        screen.schedule_frame(&frame_assembler.get_frame());
        if let Ok(elapsed) = time.elapsed() {
            if Duration::from_secs_f32(sleep_duration) > elapsed {
                sleep(Duration::from_secs_f32(sleep_duration) - elapsed);
            }
        } else {
            return;
        }
        _frame_count += 1;
    }
}
