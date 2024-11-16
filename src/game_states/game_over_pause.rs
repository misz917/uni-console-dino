use super::game_state::GameState;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GameOverPause;
impl GameState for GameOverPause {
    fn handle_input(
        &mut self,
        view: &mut crate::view::View,
        input: char,
        state_changer: &mut Option<super::game_state::GameStateEnum>,
        task_scheduler: &mut crate::task_scheduler::TaskScheduler,
        resources: &mut std::collections::HashMap<String, crate::utils::Value>,
    ) {
        todo!()
    }

    fn on_enter(
        &mut self,
        view: &mut crate::view::View,
        task_scheduler: &mut crate::task_scheduler::TaskScheduler,
        resources: &mut std::collections::HashMap<String, crate::utils::Value>,
    ) {
        todo!()
    }

    fn on_exit(
        &mut self,
        view: &mut crate::view::View,
        task_scheduler: &mut crate::task_scheduler::TaskScheduler,
        resources: &mut std::collections::HashMap<String, crate::utils::Value>,
    ) {
        todo!()
    }

    fn each_frame(
        &mut self,
        view: &mut crate::view::View,
        state_changer: &mut Option<super::game_state::GameStateEnum>,
        task_scheduler: &mut crate::task_scheduler::TaskScheduler,
        resources: &mut std::collections::HashMap<String, crate::utils::Value>,
    ) {
        todo!()
    }
}
