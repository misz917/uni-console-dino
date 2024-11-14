use crate::{
    drawable_object::{DrawableObject, Label, Rectangle},
    movement_functions,
    utils::XY,
    view::{MovementFunction, View},
    WINDOW_RESOLUTION,
};

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
    );
    fn on_enter(&mut self, view: &mut View);
    fn on_exit(&mut self, view: &mut View);
    fn each_frame(&mut self, view: &mut View, state_changer: &mut Option<GameStateEnum>);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Menu;
impl GameState for Menu {
    fn handle_input(
        &mut self,
        _view: &mut View,
        input: char,
        state_changer: &mut Option<GameStateEnum>,
    ) {
        match input {
            _ => *state_changer = Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
        }
    }

    fn on_enter(&mut self, view: &mut View) {
        view.insert_object(
            "press_to_play_label",
            false,
            DrawableObject::Label(Label::new("Press any button to start")),
            XY::new(
                (WINDOW_RESOLUTION.x / 2 - 12) as i32,
                (WINDOW_RESOLUTION.y - 3) as i32,
            ),
            None,
        );
        view.insert_asset(
            "title_sign",
            false,
            "welcome_screen.txt",
            XY::new(
                (WINDOW_RESOLUTION.x / 2 - 34) as i32,
                (WINDOW_RESOLUTION.y / 2 - 5) as i32,
            ),
            None,
        );
    }

    fn on_exit(&mut self, view: &mut View) {
        view.remove_object("title_sign");
        view.remove_object("press_to_play_label");
    }

    fn each_frame(&mut self, _view: &mut View, _state_changer: &mut Option<GameStateEnum>) {
        return;
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MainGameLoop;
impl GameState for MainGameLoop {
    fn handle_input(
        &mut self,
        view: &mut View,
        input: char,
        _state_changer: &mut Option<GameStateEnum>,
    ) {
        match input {
            'w' => { // jump
                if view.check_for_collision_between("player", "invisible_floor") {
                    view.replace_movement_function(
                        "player",
                        Some(MovementFunction::new(movement_functions::jump)),
                    );
                }
            }
            's' => { // teleport to floor
                view.replace_movement_function("player", None);
            }
            _ => ()
        }
    }

    fn on_enter(&mut self, view: &mut View) {
        view.insert_object(
            "invisible_floor",
            false,
            DrawableObject::Rectangle(Rectangle::new(XY::new(WINDOW_RESOLUTION.x, 1), '$')),
            XY::new(0, (WINDOW_RESOLUTION.y - 4) as i32),
            None,
        );
        view.insert_asset("player", true, "dino_running.txt", XY::new(4, 32), None);
    }

    fn on_exit(&mut self, view: &mut View) {
        view.remove_object("invisible_floor");
    }

    fn each_frame(&mut self, view: &mut View, state_changer: &mut Option<GameStateEnum>) {
        if view.check_for_collision("player") {
            *state_changer = Some(GameStateEnum::GameOver(Box::new(GameOver)));
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GameOver;
impl GameState for GameOver {
    fn handle_input(
        &mut self,
        _view: &mut View,
        _input: char,
        _state_changer: &mut Option<GameStateEnum>,
    ) {
        return;
    }

    fn on_enter(&mut self, _view: &mut View) {
        return;
    }

    fn on_exit(&mut self, _view: &mut View) {
        return;
    }

    fn each_frame(&mut self, _view: &mut View, _state_changer: &mut Option<GameStateEnum>) {
        return;
    }
}
