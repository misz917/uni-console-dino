use super::{
    game_state::{GameState, GameStateEnum},
    main_game_loop::MainGameLoop,
};
use crate::{
    drawable_object::{DrawableObject, Label},
    task_scheduler::TaskScheduler,
    utils::XY,
    view::View,
    WINDOW_RESOLUTION,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Menu;
impl GameState for Menu {
    fn handle_input(
        &mut self,
        _view: &mut View,
        input: char,
        state_changer: &mut Option<GameStateEnum>,
        _task_scheduler: &mut TaskScheduler,
    ) {
        match input {
            _ => *state_changer = Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
        }
    }

    fn on_enter(&mut self, view: &mut View, _task_scheduler: &mut TaskScheduler) {
        view.insert_object(
            "press_to_play_label",
            false,
            DrawableObject::Label(Label::new("Press any button to start")),
            XY::new(
                (WINDOW_RESOLUTION.x / 2 - 12) as i32,
                (WINDOW_RESOLUTION.y - 3) as i32,
            ),
            None,
        );
        view.insert_asset(
            "title_sign",
            false,
            "welcome_screen.txt",
            XY::new(
                (WINDOW_RESOLUTION.x / 2 - 34) as i32,
                (WINDOW_RESOLUTION.y / 2 - 5) as i32,
            ),
            None,
        );
    }

    fn on_exit(&mut self, view: &mut View, _task_scheduler: &mut TaskScheduler) {
        view.remove_object("title_sign");
        view.remove_object("press_to_play_label");
    }

    fn each_frame(
        &mut self,
        _view: &mut View,
        _state_changer: &mut Option<GameStateEnum>,
        _task_scheduler: &mut TaskScheduler,
    ) {
        return;
    }
}