use crate::{game_states::GameStateEnum, view::View};
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    time::{Duration, Instant},
};

#[derive(PartialEq, Eq, Ord, Clone)]
pub struct Task {
    function: fn(&mut View, i32) -> Option<Task>, // task may return a follow-up task
    scheduled_time: Instant,
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
        function: fn(&mut View, i32) -> Option<Task>,
        scheduled_time: Duration,
        game_state: Option<GameStateEnum>,
        param: i32,
    ) -> Self {
        Task {
            function,
            scheduled_time: Instant::now() + scheduled_time,
            game_state,
            param,
        }
    }

    pub fn execute(&self, view: &mut View, param: i32) -> Option<Task> {
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

        Some(self.tasks.pop().unwrap().0)
    }

    pub fn schedule(&mut self, new_task: Task) {
        self.tasks.push(Reverse(new_task));
    }
}
