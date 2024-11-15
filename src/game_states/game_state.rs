use std::collections::HashMap;

use super::{game_over::GameOver, main_game_loop::MainGameLoop, menu::Menu};
use crate::{task_scheduler::TaskScheduler, utils::Value, view::View};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum GameStateEnum {
    Menu(Box<Menu>),
    MainGameLoop(Box<MainGameLoop>),
    GameOver(Box<GameOver>),
}
impl GameStateEnum {
    pub fn as_state(&mut self) -> &mut dyn GameState {
        match self {
            GameStateEnum::Menu(state) => state.as_mut(),
            GameStateEnum::MainGameLoop(state) => state.as_mut(),
            GameStateEnum::GameOver(state) => state.as_mut(),
        }
    }

    pub fn variant_eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (GameStateEnum::Menu(_), GameStateEnum::Menu(_))
                | (
                    GameStateEnum::MainGameLoop(_),
                    GameStateEnum::MainGameLoop(_)
                )
                | (GameStateEnum::GameOver(_), GameStateEnum::GameOver(_))
        )
    }
}

pub trait GameState {
    fn handle_input(
        &mut self,
        view: &mut View,
        input: char,
        state_changer: &mut Option<GameStateEnum>,
        task_scheduler: &mut TaskScheduler,
        resources: &mut HashMap<String, Value>,
    );
    fn on_enter(
        &mut self,
        view: &mut View,
        task_scheduler: &mut TaskScheduler,
        resources: &mut HashMap<String, Value>,
    );
    fn on_exit(
        &mut self,
        view: &mut View,
        task_scheduler: &mut TaskScheduler,
        resources: &mut HashMap<String, Value>,
    );
    fn each_frame(
        &mut self,
        view: &mut View,
        state_changer: &mut Option<GameStateEnum>,
        task_scheduler: &mut TaskScheduler,
        resources: &mut HashMap<String, Value>,
    );
}
