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
use std::collections::HashMap;

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
        match input {
            'w' => {
                let option = _resources.get_mut("selected_option");
                if let Some(selected_option) = option {
                    match selected_option {
                        Value::I32(value) => *value += 1,
                        _ => panic!(),
                    }
                }
            }
            's' => {}
            'a' => {}
            _ => *state_changer = Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
        }
    }

    fn on_enter(
        &mut self,
        view: &mut View,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        _resources.insert("selected_option".to_owned(), Value::I32(0));

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
        view: &mut View,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        view.remove_object("title_sign");
        view.remove_object("press_to_play_label");
        _resources.remove("selected_option");
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
