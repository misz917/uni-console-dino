use super::{
    game_over::GameOver,
    game_state::{GameState, GameStateEnum},
};
use crate::{
    asset_server,
    drawable_object::{DrawableObject, Rectangle},
    movement_functions, task_functions,
    task_scheduler::{Task, TaskScheduler},
    utils::XY,
    view::{MovementFunction, View},
    WINDOW_RESOLUTION,
};
use std::{panic::AssertUnwindSafe, time::Duration};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MainGameLoop;
impl GameState for MainGameLoop {
    fn handle_input(
        &mut self,
        view: &mut View,
        input: char,
        _state_changer: &mut Option<GameStateEnum>,
        task_scheduler: &mut TaskScheduler,
    ) {
        match input {
            'w' => {
                // jump
                if view.check_for_collision_between("player", "invisible_floor") {
                    view.replace_movement_function(
                        "player",
                        Some(MovementFunction::new(movement_functions::jump)),
                    );
                }
            }
            's' => {
                // teleport to floor
                view.replace_movement_function("player", None);
                view.insert_asset("smoke", false, "landing_smoke.txt", XY::new(0, 36), None);
                task_scheduler.schedule(Task::new(
                    task_functions::remove_smoke,
                    Duration::from_secs_f32(0.50),
                    None,
                    0,
                ));
            }
            _ => (),
        }
    }

    fn on_enter(&mut self, view: &mut View, task_scheduler: &mut TaskScheduler) {
        let task = Task::new(
            task_functions::spawn_vase,
            Duration::from_secs(1),
            Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
            0,
        );
        task_scheduler.schedule(task);

        view.insert_object(
            "invisible_floor",
            false,
            DrawableObject::Rectangle(Rectangle::new(XY::new(WINDOW_RESOLUTION.x, 1), '$')),
            XY::new(0, (WINDOW_RESOLUTION.y - 4) as i32),
            None,
        );
        view.insert_asset("player", true, "dino_running.txt", XY::new(4, 32), None);
    }

    fn on_exit(&mut self, view: &mut View, _task_scheduler: &mut TaskScheduler) {
        view.remove_object("invisible_floor");
    }

    fn each_frame(
        &mut self,
        view: &mut View,
        state_changer: &mut Option<GameStateEnum>,
        _task_scheduler: &mut TaskScheduler,
    ) {
        if view.check_for_collision("player") {
            *state_changer = Some(GameStateEnum::GameOver(Box::new(GameOver)));
        }
    }
}
