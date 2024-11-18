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
            _ => *state_changer = Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
        }
    }

    fn on_enter(
        &mut self,
        view: &mut View,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
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
        let text = "Press any button to start";
        view.insert_object(
            "press_to_play_label",
            2,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                (WINDOW_RESOLUTION.y - 3) as i32,
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
