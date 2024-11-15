use super::{
    game_over::GameOver,
    game_state::{GameState, GameStateEnum},
};
use crate::{
    drawable_object::{DrawableObject, Rectangle},
    task_scheduler::{Task, TaskScheduler},
    utils::{Value, XY},
    view::{MovementFunction, View},
    SPEED, SPEEDUP_RATE, WINDOW_RESOLUTION,
};
use rand::Rng;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MainGameLoop;
impl GameState for MainGameLoop {
    fn handle_input(
        &mut self,
        view: &mut View,
        input: char,
        _state_changer: &mut Option<GameStateEnum>,
        task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        match input {
            'w' => {
                // jump
                if view.check_for_collision_between("player", "invisible_floor") {
                    view.replace_movement_function(
                        "player",
                        Some(MovementFunction::new(player_jump)),
                    );
                }
            }
            's' => {
                // teleport to floor
                view.replace_movement_function("player", None);
                view.insert_asset("smoke", false, "landing_smoke.txt", XY::new(0, 36), None);
                task_scheduler.schedule(Task::new(
                    remove_smoke,
                    Duration::from_secs_f32(0.50),
                    None,
                    0,
                ));
            }
            _ => (),
        }
    }

    fn on_enter(
        &mut self,
        view: &mut View,
        task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        _resources.remove_entry("start_time");
        _resources.insert("start_time".to_owned(), Value::Instant(Instant::now()));
        {
            let mut speed = SPEED.lock().unwrap();
            *speed = 1.0;
        }

        let obstacle_spawner = Task::new(
            spawn_obstacle,
            Duration::from_secs(1),
            Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
            0,
        );
        task_scheduler.schedule(obstacle_spawner);
        task_scheduler.schedule(spawn_tree(view, 0).unwrap());

        view.insert_asset("player", true, "dino_running.txt", XY::new(4, 32), None);
        view.insert_object(
            "invisible_floor",
            false,
            DrawableObject::Rectangle(Rectangle::new(XY::new(WINDOW_RESOLUTION.x, 1), '$')),
            XY::new(0, (WINDOW_RESOLUTION.y - 4) as i32),
            None,
        );
        view.insert_asset(
            "ground",
            false,
            "ground.txt",
            XY::new(0, WINDOW_RESOLUTION.y as i32 - 3),
            None,
        );
    }

    fn on_exit(
        &mut self,
        view: &mut View,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        view.remove_object("invisible_floor");
        view.remove_object("player");
        view.remove_object("vase");
        view.remove_object("bird");
        view.remove_object("ground");
        view.remove_object("tree");
    }

    fn each_frame(
        &mut self,
        view: &mut View,
        state_changer: &mut Option<GameStateEnum>,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        if view.check_for_collision("player") {
            *state_changer = Some(GameStateEnum::GameOver(Box::new(GameOver)));
        }
        let mut speed = SPEED.lock().unwrap();
        *speed *= SPEEDUP_RATE;
    }
}

fn spawn_obstacle(view: &mut View, _param: i32) -> Option<Task> {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.7) {
        view.insert_asset(
            "vase",
            true,
            "vase.txt",
            XY::new(150, 33),
            Some(MovementFunction::new(obstacle_move_left)),
        );
    } else {
        let altitude = rng.gen_range(-1..=1) * 5;
        view.insert_asset(
            "bird",
            true,
            "bird_flying.txt",
            XY::new(150, 26 + altitude),
            Some(MovementFunction::new(obstacle_move_left)),
        );
    }

    let speed = SPEED.lock().unwrap();
    let cooldown = rng.gen_range(1.2..3.0) / *speed;
    let follow_up_task = Task::new(
        spawn_obstacle,
        Duration::from_secs_f32(cooldown),
        Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
        _param,
    );
    return Some(follow_up_task);
}

pub fn obstacle_move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let speed = SPEED.lock().unwrap();
    let new_x = original_position.x - (*speed * 30.0 * time) as i32;
    let new_y = original_position.y;

    return XY::new(new_x, new_y);
}

pub fn player_jump(original_position: XY<i32>, time: f32) -> XY<i32> {
    let func = |x: f32| -(x / 2.0 - 3.18).powf(2.0) + 10.0;
    let mut difference = func(time * 8.0) as i32;
    if difference < 0 {
        difference = 0;
    }
    let new_x = original_position.x;
    let new_y = original_position.y - difference;

    return XY::new(new_x, new_y);
}

pub fn tree_move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let new_x = original_position.x - (13.0 * time) as i32;
    let new_y = original_position.y;

    return XY::new(new_x, new_y);
}

fn remove_smoke(view: &mut View, _param: i32) -> Option<Task> {
    view.remove_object("smoke");
    return None;
}

fn spawn_tree(view: &mut View, _param: i32) -> Option<Task> {
    view.insert_asset(
        "tree",
        false,
        "tree.txt",
        XY::new(175, 25),
        Some(MovementFunction::new(tree_move_left)),
    );
    let follow_up_task = Task::new(
        spawn_tree,
        Duration::from_secs(23),
        Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
        0,
    );
    return Some(follow_up_task);
}

fn spawn_sun(view: &mut View, _param: i32) -> Option<Task> {
    return None;
}

fn spawn_clouds(view: &mut View, _param: i32) -> Option<Task> {
    return None;
}

fn spawn_grass(view: &mut View, _param: i32) -> Option<Task> {
    return None;
}
