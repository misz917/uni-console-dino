use crate::{game_states::States, view::View};

pub struct GameStateManager {
    active_state: States,
}
impl GameStateManager {
    pub fn new(default_state: States) -> Self {
        GameStateManager {
            active_state: default_state,
        }
    }

    pub fn switch_state(&mut self, view: &mut View, new_state: States) {
        self.active_state.as_state().on_exit(view);
        self.active_state = new_state;
        self.active_state.as_state().on_enter(view);
    }

    pub fn handle_input(&mut self, view: &mut View, input: char) {
        self.active_state.as_state().handle_input(view, input);
    }

    fn every_frame(&mut self, view: &mut View) {
        self.active_state.as_state().every_frame(view);
    }

    pub fn handle_objects(&mut self, view: &mut View) {
        self.every_frame(view);
    }

    pub fn force_enter_run(&mut self, view: &mut View) {
        self.active_state.as_state().on_enter(view);
    }
}
