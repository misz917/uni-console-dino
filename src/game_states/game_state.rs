use super::{game_over_screen::GameOverScreen, main_game_loop::MainGameLoop, menu::Menu};
use crate::{task_scheduler::TaskScheduler, utils::Value, view::View};
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum GameStateEnum {
    Menu(Box<Menu>),
    MainGameLoop(Box<MainGameLoop>),
    GameOverScreen(Box<GameOverScreen>),
}
impl GameStateEnum {
    pub fn as_state(&mut self) -> &mut dyn GameState {
        match self {
            GameStateEnum::Menu(state) => state.as_mut(),
            GameStateEnum::MainGameLoop(state) => state.as_mut(),
            GameStateEnum::GameOverScreen(state) => state.as_mut(),
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
                | (
                    GameStateEnum::GameOverScreen(_),
                    GameStateEnum::GameOverScreen(_)
                )
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
