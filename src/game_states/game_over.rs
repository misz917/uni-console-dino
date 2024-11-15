use super::game_state::{GameState, GameStateEnum};
use crate::{task_scheduler::TaskScheduler, view::View};

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
        return;
    }

    fn on_enter(&mut self, _view: &mut View, _task_scheduler: &mut TaskScheduler) {
        return;
    }

    fn on_exit(&mut self, _view: &mut View, _task_scheduler: &mut TaskScheduler) {
        return;
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
