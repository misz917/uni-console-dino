use crate::{
    game_states::{Menu, States},
    view::View,
};

pub struct GameStateManager {
    active_state: States,
}
impl GameStateManager {
    pub fn new() -> Self {
        GameStateManager {
            active_state: States::Menu(Box::new(Menu)),
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
}
