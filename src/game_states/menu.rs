use super::{
    game_state::{GameState, GameStateEnum},
    main_game_loop::MainGameLoop,
};
use crate::{
    drawable_objects::{drawable_object::DrawableObject, label::Label},
    task_scheduler::TaskScheduler,
    utils::{Value, XY},
    view::View,
    WINDOW_RESOLUTION,
};
use std::{collections::HashMap, process::exit};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Menu;
impl GameState for Menu {
    fn handle_input(
        &mut self,
        _view: &mut View,
        input: char,
        state_changer: &mut Option<GameStateEnum>,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        let y;
        match _resources.get_mut("selected_option").unwrap() {
            Value::I32(value) => {
                match input {
                    'w' => *value += 2,
                    's' => *value += 1,
                    'a' => match *value {
                        0 => {
                            *state_changer =
                                Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop)))
                        }
                        1 => {}
                        2 => exit(0),
                        _ => (),
                    },
                    _ => (),
                }
                *value = *value % 3;
                y = *value;
            }
            _ => panic!(),
        }

        _view.remove_object("pointer");
        _view.insert_object(
            "pointer",
            -1,
            false,
            DrawableObject::Label(Label::new(">---------------------<")),
            XY::new(
                (WINDOW_RESOLUTION.x - 24) as i32 / 2,
                (WINDOW_RESOLUTION.y - (14 - y * 2) as usize) as i32,
            ),
            None,
        );
    }

    fn on_enter(
        &mut self,
        view: &mut View,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        _resources.insert("selected_option".to_owned(), Value::I32(0));

        view.insert_object(
            "pointer",
            -1,
            false,
            DrawableObject::Label(Label::new(">---------------------<")),
            XY::new(
                (WINDOW_RESOLUTION.x - 24) as i32 / 2,
                (WINDOW_RESOLUTION.y - 14) as i32,
            ),
            None,
        );

        view.insert_asset(
            "title_sign",
            1,
            false,
            "welcome_screen.txt",
            XY::new(
                ((WINDOW_RESOLUTION.x - 73) / 2) as i32,
                (WINDOW_RESOLUTION.y / 2 - 8) as i32,
            ),
            None,
        );

        let text = "Start game";
        view.insert_object(
            "start_game_label",
            2,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                (WINDOW_RESOLUTION.y - 14) as i32,
            ),
            None,
        );

        let text = "Change color scheme";
        view.insert_object(
            "change_color_scheme_label",
            2,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                (WINDOW_RESOLUTION.y - 12) as i32,
            ),
            None,
        );

        let text = "Exit game";
        view.insert_object(
            "exit_game_label",
            2,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                (WINDOW_RESOLUTION.y - 10) as i32,
            ),
            None,
        );

        let text = "W = up, S = down, A = confirm";
        view.insert_object(
            "exit_game_label",
            2,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                (WINDOW_RESOLUTION.y - 2) as i32,
            ),
            None,
        );
    }

    fn on_exit(
        &mut self,
        _view: &mut View,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        _resources.remove("selected_option");
        _view.remove_object("*");
    }

    fn each_frame(
        &mut self,
        _view: &mut View,
        _state_changer: &mut Option<GameStateEnum>,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        return;
    }
}
