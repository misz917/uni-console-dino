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

pub struct GameStateManager {
    // active_state:
}
