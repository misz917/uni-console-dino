use crate::{game_states::game_state::GameStateEnum, view::View};
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    time::{Duration, Instant},
};

#[derive(PartialEq, Eq, Ord, Clone, Debug)]
pub struct Task {
    function: fn(&mut View) -> Option<Task>, // task may return a follow-up task
    scheduled_time: Instant,
    pub game_state: Option<GameStateEnum>, // specifies to which game state task belongs to, none = no restraints
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.scheduled_time.partial_cmp(&other.scheduled_time)
    }
}
impl Task {
    pub fn new(
        function: fn(&mut View) -> Option<Task>,
        scheduled_time: Duration,
        game_state: Option<GameStateEnum>,
    ) -> Self {
        Task {
            function,
            scheduled_time: Instant::now() + scheduled_time,
            game_state,
        }
    }

    pub fn execute(&self, view: &mut View) -> Option<Task> {
        (self.function)(view)
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

    pub fn clear(&mut self) {
        self.tasks.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_states::{game_state::GameStateEnum, menu::Menu};
    use std::time::{Duration, Instant};

    fn mock_task_fn(_view: &mut View) -> Option<Task> {
        None
    }

    #[test]
    fn test_task_creation() {
        let scheduled_time = Duration::from_secs(5);
        let game_state = Some(GameStateEnum::Menu(Box::new(Menu)));

        let task = Task::new(mock_task_fn, scheduled_time, game_state.clone());

        assert_eq!(task.game_state, game_state);
        assert!(task.scheduled_time > Instant::now());
    }

    #[test]
    fn test_task_execution() {
        let scheduled_time = Duration::from_secs(1);
        let game_state = Some(GameStateEnum::Menu(Box::new(Menu)));

        let task = Task::new(mock_task_fn, scheduled_time, game_state);

        let mut view = View::new("assets", '.');

        let follow_up_task = task.execute(&mut view);

        assert_eq!(follow_up_task, None);
    }

    #[test]
    fn test_task_execution_with_follow_up() {
        fn mock_task_with_follow_up(_view: &mut View) -> Option<Task> {
            Some(Task::new(mock_task_fn, Duration::from_secs(0), None))
        }

        let scheduled_time = Duration::from_secs(1);
        let task = Task::new(mock_task_with_follow_up, scheduled_time, None);

        let mut view = View::new("assets", '.');

        let follow_up_task = task.execute(&mut view);

        assert!(follow_up_task.is_some());
    }

    #[test]
    fn test_task_scheduler_creation() {
        let scheduler = TaskScheduler::new();
        assert!(scheduler.tasks.is_empty());
    }

    #[test]
    fn test_task_scheduler_schedule() {
        let mut scheduler = TaskScheduler::new();

        let scheduled_time = Duration::from_secs(2);
        let task = Task::new(mock_task_fn, scheduled_time, None);

        scheduler.schedule(task);

        assert_eq!(scheduler.tasks.len(), 1);
    }

    #[test]
    fn test_task_scheduler_get_task() {
        let mut scheduler = TaskScheduler::new();

        let scheduled_time = Duration::from_secs(0);
        let task = Task::new(mock_task_fn, scheduled_time, None);

        scheduler.schedule(task);

        let fetched_task = scheduler.get_task();
        assert!(fetched_task.is_some());
    }

    #[test]
    fn test_task_scheduler_execute_task() {
        let mut scheduler = TaskScheduler::new();

        let scheduled_time = Duration::from_secs(1);
        let task = Task::new(mock_task_fn, scheduled_time, None);

        scheduler.schedule(task);

        std::thread::sleep(Duration::from_secs(1));

        let task = scheduler.get_task().unwrap();
        let mut view = View::new("assets", '.');
        let follow_up_task = task.execute(&mut view);

        assert_eq!(follow_up_task, None);
    }

    #[test]
    fn test_task_scheduler_does_not_execute_task_before_due() {
        let mut scheduler = TaskScheduler::new();

        let scheduled_time = Duration::from_secs(1);
        let task = Task::new(mock_task_fn, scheduled_time, None);

        scheduler.schedule(task);

        assert_eq!(scheduler.get_task(), None);
    }

    #[test]
    fn test_task_scheduler_clear() {
        let mut scheduler = TaskScheduler::new();

        let task1 = Task::new(mock_task_fn, Duration::from_secs(1), None);
        let task2 = Task::new(mock_task_fn, Duration::from_secs(2), None);

        scheduler.schedule(task1);
        scheduler.schedule(task2);

        assert_eq!(scheduler.tasks.len(), 2);

        scheduler.clear();

        assert_eq!(scheduler.tasks.len(), 0);
    }
}
