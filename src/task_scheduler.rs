use std::collections::BTreeSet;

#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub struct Task(fn());
impl Task {
    pub fn execute(&self) {
        (self.0)()
    }
}

pub struct TaskScheduler {
    tasks_queue: BTreeSet<Task>,
}
impl TaskScheduler {
    pub fn new() -> Self {
        TaskScheduler {
            tasks_queue: BTreeSet::new(),
        }
    }

    pub fn pop(&mut self) -> Option<Task> {
        self.tasks_queue.pop_first()
    }
}
