use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    time::{Duration, Instant},
};

use crate::view::View;

#[derive(PartialEq, Eq, Ord, Clone)]
pub struct Task {
    function: fn(&mut View),
    pub scheduled_time: Instant,
    repeat_delay: Option<Duration>, // none = no repeat
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.scheduled_time.partial_cmp(&other.scheduled_time)
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
        if let Some(task) = self.tasks.peek() {
            if Instant::now() >= task.0.scheduled_time {
                return true;
            }
        }
        return false;
    }

    pub fn get_task(&mut self) -> Option<Task> {
        if self.task_available() {
            if let Some(task) = self.tasks.pop() {
                if task.0.repeat_delay.is_some() {
                    let mut new_task = task.0.clone();
                    new_task.scheduled_time = Instant::now() + new_task.repeat_delay.unwrap();
                    self.schedule(new_task);
                }
                return Some(task.0);
            }
        }
        return None;
    }

    pub fn schedule(&mut self, new_task: Task) {
        self.tasks.push(Reverse(new_task));
    }
}
