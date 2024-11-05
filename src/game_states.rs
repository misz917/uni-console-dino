use crate::{
    movement_functions,
    view::{MovementFunction, View},
};

pub enum States {
    Menu(Box<Menu>),
    MainGameLoop(Box<MainGameLoop>),
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
