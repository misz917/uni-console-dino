use std::time::Duration;
use crate::{
    game_states::{GameStateEnum, MainGameLoop},
    movement_functions,
    task_scheduler::Task,
    utils::XY,
    view::{MovementFunction, View}
};

pub fn spawn_vase(view: &mut View, _param: i32) -> Option<Task> {
    view.insert_asset(
        "vase",
        true,
        "vase.txt",
        XY::new(150, 33),
        Some(MovementFunction::new(movement_functions::move_left)),
    );
    
    let follow_up_task = Task::new(
        spawn_vase,
        Duration::from_secs(1),
        Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))), _param);
    return Some(follow_up_task);
}
