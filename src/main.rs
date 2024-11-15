use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self},
};

pub mod animation;
pub mod asset_server;
pub mod bitmap;
pub mod bitmap_buffer;
pub mod collision_detector;
pub mod drawable_object;
pub mod frame_assembler;
pub mod game_controller;
pub mod game_states;
pub mod movement_functions;
pub mod task_functions;
pub mod task_scheduler;
pub mod terminal_screen;
pub mod utils;
pub mod view;
pub mod window;

use crate::{
    bitmap::{Bitmap, BitmapPrinter},
    bitmap_buffer::BitmapBuffer,
    game_controller::GameController,
    game_states::{game_state::GameStateEnum, menu::Menu},
    task_scheduler::TaskScheduler,
    terminal_screen::{TerminalHelper, TerminalScreen},
    utils::XY,
    view::View,
    window::{GnomeTerminal, Terminal, WindowCreator},
};

const BORDER_WIDTH: XY<usize> = XY::new(2, 1);
const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 40);
const FPS_LIMIT: f32 = 40.0; // may be buggy above ~46

fn main() {
    let gnome_window = GnomeTerminal::new();
    WindowCreator::create_separate_window(WINDOW_RESOLUTION, BORDER_WIDTH, &gnome_window);
    gnome_window.set_raw_mode();
    TerminalHelper::prepare_terminal();

    let (tx, rx): (Sender<char>, Receiver<char>) = mpsc::channel();
    thread::spawn(move || loop {
        let input = gnome_window.read_key();
        if let Some(pressed_key) = input {
            tx.send(pressed_key).unwrap();
        }
    });

    let asset_path = "/home/user/Codes/GithubRepos/uni-console-dino/src/assets/";
    let bitmap_buffer = BitmapBuffer::new(&Bitmap::new(WINDOW_RESOLUTION, '$'));
    let view = View::new(asset_path, ' ');
    let screen = TerminalScreen::new(bitmap_buffer, BitmapPrinter, BORDER_WIDTH);
    let task_scheduler = TaskScheduler::new();
    let mut game_controller = GameController::new(
        view,
        screen,
        rx,
        GameStateEnum::Menu(Box::new(Menu)),
        task_scheduler,
    );
    game_controller.run();
}
