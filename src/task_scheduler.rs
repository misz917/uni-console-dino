use std::{
    collections::BTreeSet,
    time::{Duration, SystemTime},
};

#[derive(PartialEq, Eq)]
pub struct Task {
    function: fn(),
    execution_time: SystemTime,
}
impl Task {
    pub fn execute(&self) {
        (self.function)()
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.execution_time.partial_cmp(&other.execution_time)
    }
}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.execution_time.cmp(&other.execution_time)
    }
}

pub struct TaskScheduler {
    tasks_queue: BTreeSet<Task>,
    init_time: SystemTime,
}
impl TaskScheduler {
    pub fn new() -> Self {
        TaskScheduler {
            tasks_queue: BTreeSet::new(),
            init_time: SystemTime::now(),
        }
    }

    pub fn pop(&mut self) -> Option<Task> {
        self.tasks_queue.pop_first()
    }

    pub fn push(&mut self, function: fn(), execution_time: Duration) {
        if let Some(time) = self.init_time.checked_add(execution_time) {
            let new_task = Task {
                function,
                execution_time: time,
            };
            self.tasks_queue.insert(new_task);
        }
    }
}
