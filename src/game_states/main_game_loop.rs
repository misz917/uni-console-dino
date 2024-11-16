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
use rand_distr::{Distribution, Normal};
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

        task_scheduler.schedule(spawn_obstacle(view).unwrap());
        task_scheduler.schedule(spawn_tree(view).unwrap());
        task_scheduler.schedule(spawn_sun(view).unwrap());
        task_scheduler.schedule(spawn_clouds(view).unwrap());
        task_scheduler.schedule(spawn_grass(view).unwrap());

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
        _task_scheduler.clear();
        view.remove_object("invisible_floor");
        view.remove_object("player");
        view.remove_object("vase");
        view.remove_object("bird");
        view.remove_object("ground");
        view.remove_object("tree");
        view.remove_object("sun");
        view.remove_object("smoke");
        view.remove_object("cloud");
        view.remove_object("grass");
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

fn spawn_obstacle(view: &mut View) -> Option<Task> {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.7) {
        view.insert_asset(
            "vase",
            true,
            "vase.txt",
            XY::new(170, 33),
            Some(MovementFunction::new(obstacle_move_left)),
        );
    } else {
        let altitude = rng.gen_range(-1..=1) * 5;
        view.insert_asset(
            "bird",
            true,
            "bird_flying.txt",
            XY::new(170, 26 + altitude),
            Some(MovementFunction::new(obstacle_move_left)),
        );
    }

    let speed = SPEED.lock().unwrap();
    let cooldown = rng.gen_range(1.5..3.5) / *speed;
    let follow_up_task = Task::new(
        spawn_obstacle,
        Duration::from_secs_f32(cooldown),
        Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
    );
    return Some(follow_up_task);
}

fn obstacle_move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let speed = SPEED.lock().unwrap();
    let new_x = original_position.x - (*speed * 30.0 * time) as i32;
    let new_y = original_position.y;

    return XY::new(new_x, new_y);
}

fn player_jump(original_position: XY<i32>, time: f32) -> XY<i32> {
    let func = |x: f32| -(x / 2.0 - 3.18).powf(2.0) + 10.0;
    let mut difference = func(time * 8.0) as i32;
    if difference < 0 {
        difference = 0;
    }
    let new_x = original_position.x;
    let new_y = original_position.y - difference;

    return XY::new(new_x, new_y);
}

fn remove_smoke(view: &mut View) -> Option<Task> {
    view.remove_object("smoke");
    return None;
}

fn spawn_tree(view: &mut View) -> Option<Task> {
    view.insert_asset(
        "tree",
        false,
        "tree.txt",
        XY::new(175, 25),
        Some(MovementFunction::new(tree_move_left)),
    );
    let delay = rand::thread_rng().gen_range(10..30);
    let follow_up_task = Task::new(
        spawn_tree,
        Duration::from_secs(delay),
        Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
    );
    return Some(follow_up_task);
}

fn tree_move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let new_x = original_position.x - (13.0 * time) as i32;
    let new_y = original_position.y;

    return XY::new(new_x, new_y);
}

fn spawn_sun(view: &mut View) -> Option<Task> {
    view.insert_asset(
        "sun",
        false,
        "sun.txt",
        XY::new(165, 2),
        Some(MovementFunction::new(sun_move_left)),
    );
    let follow_up_task = Task::new(
        spawn_sun,
        Duration::from_secs(170),
        Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
    );
    return Some(follow_up_task);
}

fn sun_move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let new_x = original_position.x - time as i32;
    let new_y = original_position.y;

    return XY::new(new_x, new_y);
}

fn spawn_clouds(view: &mut View) -> Option<Task> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(10.0, 2.3).unwrap();

    let movement_function: MovementFunction;
    if rng.gen_bool(0.7) {
        movement_function = MovementFunction::new(cloud_slow_move_left);
    } else {
        movement_function = MovementFunction::new(cloud_fast_move_left);
    }

    let number_of_clouds = rng.gen_range(1..=3);
    for _ in 0..number_of_clouds {
        let random_y = normal.sample(&mut rng) as i32;
        let x_deviation = rng.gen_range(-2..=2);
        if rng.gen_bool(0.5) {
            view.insert_asset(
                "cloud",
                false,
                "smaller_cloud.txt",
                XY::new(165 + x_deviation, random_y),
                Some(movement_function.clone()),
            );
        } else {
            view.insert_asset(
                "cloud",
                false,
                "larger_cloud.txt",
                XY::new(165 + x_deviation, random_y),
                Some(movement_function.clone()),
            );
        }
    }

    let delay = rng.gen_range(0.5..3.5);
    let follow_up_task = Task::new(
        spawn_clouds,
        Duration::from_secs_f32(delay),
        Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
    );
    return Some(follow_up_task);
}

fn cloud_slow_move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let new_x = original_position.x - (4.0 * time) as i32;
    let new_y = original_position.y;

    return XY::new(new_x, new_y);
}

fn cloud_fast_move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let new_x = original_position.x - (5.0 * time) as i32;
    let new_y = original_position.y;

    return XY::new(new_x, new_y);
}

fn spawn_grass(view: &mut View) -> Option<Task> {
    view.insert_asset(
        "grass",
        false,
        "swaying_grass.txt",
        XY::new(163, WINDOW_RESOLUTION.y as i32 - 4),
        Some(MovementFunction::new(grass_move_left)),
    );

    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(0.1..2.0);
    let follow_up_task = Task::new(
        spawn_grass,
        Duration::from_secs_f32(delay),
        Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
    );
    return Some(follow_up_task);
}

fn grass_move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let new_x = original_position.x - (15.0 * time) as i32;
    let new_y = original_position.y;

    return XY::new(new_x, new_y);
}
