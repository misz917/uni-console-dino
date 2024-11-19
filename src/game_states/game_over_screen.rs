use super::{
    game_state::{GameState, GameStateEnum},
    menu::Menu,
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
pub struct GameOverScreen;
impl GameState for GameOverScreen {
    fn handle_input(
        &mut self,
        _view: &mut View,
        _input: char,
        _state_changer: &mut Option<GameStateEnum>,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        match _input {
            'd' => *_state_changer = Some(GameStateEnum::Menu(Box::new(Menu))),
            _ => (),
        }
    }

    fn on_enter(
        &mut self,
        _view: &mut View,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        let text = "Game over";
        _view.insert_object(
            "game_over_label",
            1,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                WINDOW_RESOLUTION.y as i32 / 2 - 4,
            ),
            None,
        );

        let start_time = _resources.get("start_time").unwrap();
        let elapsed_time = match start_time {
            Value::Instant(instant) => instant.elapsed(),
            _ => panic!(), // will never happen anyway
        };
        let score = elapsed_time.as_secs_f32();
        let text = format!("Your score: {}", (score * 100.0).round());
        _view.insert_object(
            "score_label",
            1,
            false,
            DrawableObject::Label(Label::new(&text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                WINDOW_RESOLUTION.y as i32 / 2 - 2,
            ),
            None,
        );

        let text = "Press D to go to menu";
        _view.insert_object(
            "press_button_label",
            1,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                WINDOW_RESOLUTION.y as i32 / 2,
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
        _view.remove_object("game_over_label");
        _view.remove_object("score_label");
        _view.remove_object("press_button_label");
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
