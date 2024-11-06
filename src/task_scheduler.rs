use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    time::{Duration, Instant},
};

use crate::{game_states::GameStateEnum, view::View};

#[derive(PartialEq, Eq, Ord, Clone)]
pub struct Task {
    function: fn(&mut View, i32),
    scheduled_time: Instant,
    repeat_delay: Option<Duration>,        // none = no repeat
    pub game_state: Option<GameStateEnum>, // specifies to which game state task belongs to, none = no restraints
    param: i32,                            // parameter for anything
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.scheduled_time.partial_cmp(&other.scheduled_time)
    }
}
impl Task {
    pub fn new(
        function: fn(&mut View, i32),
        scheduled_time: Duration,
        repeat_delay: Option<Duration>,
        game_state: Option<GameStateEnum>,
        param: i32,
    ) -> Self {
        Task {
            function,
            scheduled_time: Instant::now() + scheduled_time,
            repeat_delay,
            game_state,
            param,
        }
    }

    pub fn execute(&self, view: &mut View, param: i32) {
        (self.function)(view, param)
    }
}

pub struct TaskScheduler {
    tasks: BinaryHeap<Reverse<Task>>,
}
impl TaskScheduler {
    pub fn new() -> Self {
        TaskScheduler {
            tasks: BinaryHeap::new(),
        }
    }

    fn task_available(&self) -> bool {
        self.tasks
            .peek()
            .map_or(false, |task| Instant::now() >= task.0.scheduled_time)
    }

    pub fn get_task(&mut self) -> Option<Task> {
        if !self.task_available() {
            return None;
        }

        self.tasks.pop().map(|task| {
            if let Some(delay) = task.0.repeat_delay {
                let mut new_task = task.0.clone();
                new_task.scheduled_time = Instant::now() + delay;
                self.schedule(new_task);
            }
            task.0
        })
    }

    pub fn schedule(&mut self, new_task: Task) {
        self.tasks.push(Reverse(new_task));
    }
}
