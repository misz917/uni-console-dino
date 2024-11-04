use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, sleep},
    time::{Duration, SystemTime},
};

pub mod animation;
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
const FPS_LIMIT: f32 = 40.0; // buggy above ~46

#[derive(PartialEq, Eq, Clone)]
pub enum GameState {
    Menu,
    MainGame,
    GameOver,
}

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

    let asset_path = "/home/user/Codes/githubRepos/uni-console-dino/src/assets/";
    let mut view = View::new(asset_path, ' ');

    let mut game_state = GameState::Menu;
    let mut last_game_state = game_state.clone();

    let mut first_run = true;

    let mut _frame_counter: u64 = 0;
    loop {
        let timer = SystemTime::now();

        let label = DrawableObject::Label(Label::new(&format!("Frame: {}", _frame_counter)));
        view.remove_object("frame_count");
        view.insert_object("frame_count", false, label, XY::new(2, 1), None);
        handle_input(&mut view, &rx, &mut game_state, &mut last_game_state);

        if last_game_state != game_state {
            last_game_state = game_state.clone();
            first_run = true;
        }

        match game_state {
            GameState::Menu => {
                if first_run {
                    view.insert_object(
                        "start_label",
                        false,
                        DrawableObject::Label(Label::new("Press any button to start the game")),
                        XY::new(2, 2),
                        None,
                    );
                    first_run = false;
                }
            }
            GameState::MainGame => {
                if first_run {
                    insert_objects(&mut view);
                    first_run = false;
                }
                if view.check_for_collision("player") {
                    game_state = GameState::GameOver;
                }
                if _frame_counter % 120 == 0 {
                    view.insert_asset(
                        &format!("vase{}", _frame_counter),
                        true,
                        "vase.txt",
                        XY::new(WINDOW_RESOLUTION.x as i32 + 1, 33),
                        Some(MovementFunction::new(movement_functions::move_left)),
                    );
                } else if _frame_counter % 200 == 0 {
                    view.insert_asset(
                        &format!("bird{}", _frame_counter),
                        true,
                        "bird_flying.txt",
                        XY::new(WINDOW_RESOLUTION.x as i32 + 1, 23),
                        Some(MovementFunction::new(movement_functions::move_left)),
                    );
                }
            }
            GameState::GameOver => {
                ErrorDisplayer::error("Game Over");
            }
        }

        screen.schedule_frame(view.compile());
        screen.display_frame();
        enforce_fps(timer);
        _frame_counter += 1;
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
    view.insert_asset("player", true, "dino_running.txt", XY::new(5, 32), None);

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

fn handle_input(
    view: &mut View,
    rx: &Receiver<char>,
    game_state: &mut GameState,
    last_game_state: &mut GameState,
) {
    if let Ok(input) = rx.try_recv() {
        match game_state {
            GameState::Menu => {
                *game_state = GameState::MainGame;
                *last_game_state = GameState::Menu;
            }
            GameState::MainGame => match input {
                ' ' => {
                    if view.check_for_collision_between("player", "invisible_floor") {
                        view.replace_movement_function(
                            "player",
                            Some(MovementFunction::new(movement_functions::jump)),
                        );
                    }
                }
                _ => (),
            },
            GameState::GameOver => {}
        }
    }
}
