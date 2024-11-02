use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, sleep},
    time::{Duration, SystemTime},
};

pub mod asset_server;
pub mod bitmap;
pub mod bitmap_buffer;
pub mod collision_detector;
pub mod drawable_object;
pub mod frame_assembler;
pub mod movement_functions;
pub mod task_scheduler;
pub mod terminal_screen;
pub mod utils;
pub mod view;
pub mod window;

use drawable_object::{DrawableObject, Label, Rectangle};

use crate::{
    bitmap::{Bitmap, BitmapPrinter},
    bitmap_buffer::BitmapBuffer,
    terminal_screen::{TerminalHelper, TerminalScreen},
    utils::ErrorDisplayer,
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

    let asset_path = "/home/firstuser/Codes/githubRepos/uni-console-dino/src/assets/";
    let mut view = View::new(asset_path, ' ');
    insert_objects(&mut view);

    let mut frame_counter: u64 = 0;
    loop {
        let timer = SystemTime::now();

        handle_input(&mut view, &rx);

        if view.check_for_collision("player") {
            ErrorDisplayer::error("The End");
        }

        screen.schedule_frame(view.compile());
        screen.display_frame();
        enforce_fps(timer);
        frame_counter += 1;
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
    view.insert_asset("player", true, "dino.txt", XY::new(5, 32), None);

    view.insert_asset(
        "obstacle",
        true,
        "vase.txt",
        XY::new(150, 33),
        Some(MovementFunction::new(movement_functions::move_left)),
    );

    let test_label = DrawableObject::Label(Label::new("Hardcore gameplay live"));
    view.insert_object("test_label", false, test_label, XY::new(2, 1), None);

    let invisible_floor =
        DrawableObject::Rectangle(Rectangle::new(XY::new(WINDOW_RESOLUTION.x, 1), '$'));
    view.insert_object(
        "invisible_floor",
        false,
        invisible_floor,
        XY::new(0, 36),
        None,
    );
}

fn handle_input(view: &mut View, rx: &Receiver<char>) {
    if let Ok(input) = rx.try_recv() {
        match input {
            ' ' => {
                if view.check_for_collision_between("player", "invisible_floor") {
                    view.replace_movement_function(
                        "player",
                        Some(MovementFunction::new(movement_functions::jump)),
                    );
                }
            }
            _ => (),
        }
    }
}
