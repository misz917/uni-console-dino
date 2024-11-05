use crate::{
    movement_functions,
    view::{MovementFunction, View},
};

// implement a way to automatically remove objects once the game state changes
// and maybe re-insert them when the state changes back

pub trait GameState {
    fn handle_input(view: &mut View, input: char);
    fn handle_objects_loop(view: &mut View);
    fn handle_objects_once(view: &mut View);
}

pub struct Menu;
impl GameState for Menu {
    fn handle_input(view: &mut View, input: char) {
        match input {
            _ => {} // switch to main loop
        }
        todo!()
    }

    fn handle_objects_loop(view: &mut View) {
        todo!()
    }

    fn handle_objects_once(view: &mut View) {
        todo!()
    }
}

pub struct MainGameLoop;
impl GameState for MainGameLoop {
    fn handle_input(view: &mut View, input: char) {
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

    fn handle_objects_loop(view: &mut View) {
        todo!()
    }

    fn handle_objects_once(view: &mut View) {
        todo!()
    }
}

pub struct GameOver;
impl GameState for GameOver {
    fn handle_input(view: &mut View, input: char) {
        match input {
            _ => {} // switch to main loop
        }
        todo!()
    }

    fn handle_objects_loop(view: &mut View) {
        todo!()
    }

    fn handle_objects_once(view: &mut View) {
        todo!()
    }
}

pub enum States {
    Menu,
    MainGameLoop,
    GameOver,
}
impl States {
    pub fn default() -> Self {
        States::Menu
    }
}

pub struct GameStateManager {
    active_state: States,
    first_run: bool,
}
impl GameStateManager {
    pub fn new() -> Self {
        GameStateManager {
            active_state: States::default(),
            first_run: true,
        }
    }

    pub fn switch_state(&mut self, new_state: States) {
        self.first_run = true;
        self.active_state = new_state;
    }

    pub fn handle_input() {
        todo!()
    }

    fn handle_objects_once(&mut self) {
        todo!()
    }

    fn handle_objects_loop(&mut self) {
        todo!()
    }

    pub fn handle_objects(&mut self) {
        if self.first_run {
            self.handle_objects_once();
        } else {
            self.handle_objects_loop();
        }
    }
}
