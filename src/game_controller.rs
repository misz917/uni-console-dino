use std::{
    sync::mpsc::Receiver,
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::{
    bitmap::Printer,
    bitmap_buffer::BufferManager,
    drawable_object::{DrawableObject, Label},
    game_states::GameStateEnum,
    task_scheduler::TaskScheduler,
    terminal_screen::TerminalScreen,
    utils::XY,
    view::View,
    FPS_LIMIT, WINDOW_RESOLUTION,
};

pub struct GameController<B: BufferManager, P: Printer> {
    frame_counter: u64,
    view: View,
    screen: TerminalScreen<B, P>,
    rx: Receiver<char>,
    active_state: GameStateEnum,
    task_scheduler: TaskScheduler,
}
impl<B: BufferManager, P: Printer> GameController<B, P> {
    pub fn new(
        view: View,
        screen: TerminalScreen<B, P>,
        rx: Receiver<char>,
        default_game_state: GameStateEnum,
        task_scheduler: TaskScheduler,
    ) -> Self {
        GameController {
            frame_counter: 0,
            view,
            screen,
            rx,
            active_state: default_game_state,
            task_scheduler,
        }
    }

    fn display_frame_counter(&mut self) {
        let label = DrawableObject::Label(Label::new(&format!("{}", self.frame_counter)));
        self.view.remove_object("frame_count");
        self.view.insert_object(
            "frame_count",
            false,
            label,
            XY::new(
                (WINDOW_RESOLUTION.x - 1 - (f32::log10(self.frame_counter as f32)) as usize) as i32,
                (WINDOW_RESOLUTION.y - 1) as i32,
            ),
            None,
        );
    }

    pub fn run(&mut self) {
        loop {
            let timer = SystemTime::now();
            self.display_frame_counter();

            if let Ok(input) = self.rx.try_recv() {
                self.active_state
                    .as_state()
                    .handle_input(&mut self.view, input);
            }
            if let Some(task) = self.task_scheduler.get_task() {
                task.execute(&mut self.view);
            }

            self.screen.schedule_frame(self.view.compile());
            self.screen.display_frame();

            Self::enforce_fps(timer);
            self.frame_counter += 1;
        }
    }

    fn change_state(&mut self, new_state: GameStateEnum) {
        self.active_state.as_state().on_exit(&mut self.view);
        self.active_state = new_state;
        self.active_state.as_state().on_enter(&mut self.view);
    }

    fn enforce_fps(timer: SystemTime) {
        let sleep_duration = 1.0 / FPS_LIMIT;
        if let Ok(elapsed) = timer.elapsed() {
            if Duration::from_secs_f32(sleep_duration) > elapsed {
                sleep(Duration::from_secs_f32(sleep_duration) - elapsed);
            }
        }
    }
}
