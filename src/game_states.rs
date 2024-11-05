use crate::{
    movement_functions,
    view::{MovementFunction, View},
};

// implement a way to automatically remove objects once the game state changes
// and maybe re-insert them when the state changes back

pub trait GameState {
    fn handle_input(&mut self, view: &mut View, input: char);
    fn every_frame(&mut self, view: &mut View);
    fn on_enter(&mut self, view: &mut View);
    fn on_exit(&mut self, view: &mut View);
}

pub struct Menu;
impl GameState for Menu {
    fn handle_input(&mut self, view: &mut View, input: char) {
        todo!()
    }

    fn every_frame(&mut self, view: &mut View) {
        todo!()
    }

    fn on_enter(&mut self, view: &mut View) {
        todo!()
    }

    fn on_exit(&mut self, view: &mut View) {
        todo!()
    }
}

pub struct MainGameLoop;
impl GameState for MainGameLoop {
    fn handle_input(&mut self, view: &mut View, input: char) {
        match input {
            _ => {
                if view.check_for_collision_between("player", "invisible_floor") {
                    view.replace_movement_function(
                        "player",
                        Some(MovementFunction::new(movement_functions::jump)),
                    );
                }
            }
        }
    }

    fn every_frame(&mut self, view: &mut View) {
        todo!()
    }

    fn on_enter(&mut self, view: &mut View) {
        todo!()
    }

    fn on_exit(&mut self, view: &mut View) {
        todo!()
    }
}

pub struct GameOver;
impl GameState for GameOver {
    fn handle_input(&mut self, view: &mut View, input: char) {
        todo!()
    }

    fn every_frame(&mut self, view: &mut View) {
        todo!()
    }

    fn on_enter(&mut self, view: &mut View) {
        todo!()
    }

    fn on_exit(&mut self, view: &mut View) {
        todo!()
    }
}

pub enum States {
    Menu(Box<Menu>),
    MainGameLoop(Box<Menu>),
    GameOver(Box<GameOver>),
}
impl States {
    pub fn as_state(&mut self) -> &mut dyn GameState {
        match self {
            States::Menu(state) => state.as_mut(),
            States::MainGameLoop(state) => state.as_mut(),
            States::GameOver(state) => state.as_mut(),
        }
    }
}

pub struct GameStateManager {
    active_state: States,
    first_run: bool,
}
impl GameStateManager {
    pub fn new() -> Self {
        GameStateManager {
            active_state: States::Menu(Box::new(Menu)),
            first_run: true,
        }
    }

    pub fn switch_state(&mut self, view: &mut View, new_state: States) {
        self.active_state.as_state().on_exit(view);
        self.active_state = new_state;
        self.first_run = true;
        self.active_state.as_state().on_enter(view);
    }

    pub fn handle_input(&mut self, view: &mut View, input: char) {
        todo!()
    }

    fn every_frame(&mut self, view: &mut View) {
        todo!()
    }

    fn on_enter(&mut self, view: &mut View) {
        todo!()
    }

    fn on_exit(&mut self, view: &mut View) {
        todo!()
    }

    pub fn handle_objects(&mut self, view: &mut View) {
        if self.first_run {
            self.on_enter(view);
            self.first_run = false;
        } else {
            self.every_frame(view);
        }
    }
}
