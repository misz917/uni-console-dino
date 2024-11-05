use std::{
    sync::mpsc::Receiver,
    thread::{current, sleep},
    time::{Duration, SystemTime},
};

use crate::{
    bitmap::Printer,
    bitmap_buffer::BufferManager,
    drawable_object::{DrawableObject, Label},
    game_states::{GameStateEnum, MainGameLoop, Menu},
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
}
impl<B: BufferManager, P: Printer> GameController<B, P> {
    pub fn new(
        view: View,
        screen: TerminalScreen<B, P>,
        rx: Receiver<char>,
        default_game_state: GameStateEnum,
    ) -> Self {
        GameController {
            frame_counter: 0,
            view,
            screen,
            rx,
            active_state: default_game_state,
        }
    }

    pub fn run(&mut self) {
        self.active_state.as_state().on_enter(&mut self.view);
        loop {
            let timer = SystemTime::now();

            let label = DrawableObject::Label(Label::new(&format!("{}", self.frame_counter)));
            self.view.remove_object("frame_count");
            self.view.insert_object(
                "frame_count",
                false,
                label,
                XY::new(
                    (WINDOW_RESOLUTION.x - 1 - (f32::log10(self.frame_counter as f32)) as usize)
                        as i32,
                    (WINDOW_RESOLUTION.y - 1) as i32,
                ),
                None,
            );

            self.check_state_transitions();

            if let Ok(input) = self.rx.try_recv() {
                self.active_state
                    .as_state()
                    .handle_input(&mut self.view, input);
            }
            self.active_state.as_state().every_frame(&mut self.view);

            self.screen.schedule_frame(self.view.compile());
            self.screen.display_frame();

            Self::enforce_fps(timer);
            self.frame_counter += 1;
        }
    }

    fn check_state_transitions(&mut self) {
        match &mut self.active_state {
            GameStateEnum::Menu(menu) => {
                if let Ok(input) = self.rx.try_recv() {
                    self.active_state.as_state().on_exit(&mut self.view);
                    self.active_state = GameStateEnum::MainGameLoop(Box::new(MainGameLoop));
                    self.active_state.as_state().on_enter(&mut self.view);
                }
            }
            GameStateEnum::MainGameLoop(main_game_loop) => {
                if self.view.check_for_collision("player") {
                    self.active_state.as_state().on_exit(&mut self.view);
                    self.active_state = GameStateEnum::MainGameLoop(Box::new(MainGameLoop));
                    self.active_state.as_state().on_enter(&mut self.view);
                }
            }
            GameStateEnum::GameOver(game_over) => {
                if let Ok(input) = self.rx.try_recv() {
                    self.active_state.as_state().on_exit(&mut self.view);
                    self.active_state = GameStateEnum::Menu(Box::new(Menu));
                    self.active_state.as_state().on_enter(&mut self.view);
                }
            }
        }
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
