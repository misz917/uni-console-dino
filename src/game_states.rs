use crate::{
    drawable_object::{DrawableObject, Label},
    movement_functions,
    utils::XY,
    view::{MovementFunction, View},
    WINDOW_RESOLUTION,
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
        // nothing
    }

    fn on_enter(&mut self, view: &mut View) {
        view.insert_object(
            "press_to_play_label",
            false,
            DrawableObject::Label(Label::new("Press any button to start")),
            XY::new(
                (WINDOW_RESOLUTION.x / 2 - 14) as i32,
                (WINDOW_RESOLUTION.y - 3) as i32,
            ),
            None,
        );
        view.insert_asset(
            "title_sign",
            false,
            "welcome_screen.txt",
            XY::new(0, 0),
            None,
        );
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
