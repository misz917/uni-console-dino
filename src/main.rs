use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, sleep},
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
pub mod collision_detector;

use crate::{
    bitmap::{Bitmap, BitmapPrinter},
    bitmap_buffer::BitmapBuffer,
    terminal_screen::{TerminalHelper, TerminalScreen},
    utils::XY,
    view::{MovementFunction, View},
    window::{GnomeTerminal, Terminal, WindowCreator},
};

// create a settings file later
const BORDER_WIDTH: XY<usize> = XY::new(2, 1);
const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 40);
const FPS_LIMIT: f32 = 30.0; // buggy above ~46

fn main() {
    let gnome_window = GnomeTerminal::new();
    WindowCreator::create_separate_window(WINDOW_RESOLUTION, BORDER_WIDTH, &gnome_window);
    gnome_window.set_raw_mode();

    let (tx, rx): (Sender<char>, Receiver<char>) = mpsc::channel();
    thread::spawn(move || loop {
        let input = gnome_window.read_key();
        if let Some(pressed_key) = input {
            tx.send(pressed_key).unwrap();
        }
    });

    let bitmap_buffer = BitmapBuffer::new(&Bitmap::new(WINDOW_RESOLUTION, '$'));
    let mut screen = TerminalScreen::new(bitmap_buffer, BitmapPrinter, BORDER_WIDTH);
    TerminalHelper::prepare_terminal();

    // let laptop_path = "/home/user/Codes/githubRepos/uni-console-dino/src/assets/";
    let pc_path = "/home/firstuser/Codes/githubRepos/uni-console-dino/src/assets/";
    let mut view = View::new(pc_path, ' ');
    insert_objects(&mut view);

    loop {
        let timer = SystemTime::now();

        if let Ok(input) = rx.try_recv() {
            match input {
                _ => (),
            }
        }

        screen.schedule_frame(view.compile());
        screen.display_frame();
        enforce_fps(timer);
    }
}

fn enforce_fps(timer: SystemTime) {
    let sleep_duration = 1.0 / FPS_LIMIT;
    if let Ok(elapsed) = timer.elapsed() {
        if Duration::from_secs_f32(sleep_duration) > elapsed {
            sleep(Duration::from_secs_f32(sleep_duration) - elapsed);
        }
    } else {
        return;
    }
}

fn insert_objects(view: &mut View) {
    view.insert_asset(
        "dino.txt",
        XY::new(10, 25),
        Some(MovementFunction::new(movement_functions::move_right)),
    );
}