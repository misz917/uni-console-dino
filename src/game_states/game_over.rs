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
pub struct GameOver;
impl GameState for GameOver {
    fn handle_input(
        &mut self,
        _view: &mut View,
        _input: char,
        _state_changer: &mut Option<GameStateEnum>,
        _task_scheduler: &mut TaskScheduler,
    ) {
        *_state_changer = Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop)));
    }

    fn on_enter(&mut self, _view: &mut View, _task_scheduler: &mut TaskScheduler) {
        _view.insert_object(
            "game_over_label",
            false,
            DrawableObject::Label(Label::new("Game over, press any button to restart")),
            XY::new(
                WINDOW_RESOLUTION.x as i32 / 2 - 20,
                WINDOW_RESOLUTION.y as i32 / 2,
            ),
            None,
        );
    }

    fn on_exit(&mut self, _view: &mut View, _task_scheduler: &mut TaskScheduler) {
        _view.remove_object("game_over_label");
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
