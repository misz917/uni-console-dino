use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::{
    bitmap::Printer,
    bitmap_buffer::BufferManager,
    drawable_object::{DrawableObject, Label},
    game_states::GameStateManager,
    terminal_screen::TerminalScreen,
    utils::XY,
    view::View,
    FPS_LIMIT,
};

pub struct GameController<B: BufferManager, P: Printer> {
    frame_counter: u64,
    view: View,
    screen: TerminalScreen<B, P>,
    game_state_manager: GameStateManager,
}
impl<B: BufferManager, P: Printer> GameController<B, P> {
    pub fn new(
        view: View,
        screen: TerminalScreen<B, P>,
        game_state_manager: GameStateManager,
    ) -> Self {
        GameController {
            frame_counter: 0,
            view,
            screen,
            game_state_manager,
        }
    }

    pub fn run(&mut self) {
        loop {
            let timer = SystemTime::now();

            let label =
                DrawableObject::Label(Label::new(&format!("Frame: {}", self.frame_counter)));
            self.view.remove_object("frame_count");
            self.view
                .insert_object("frame_count", false, label, XY::new(2, 1), None);

            self.screen.schedule_frame(self.view.compile());
            self.screen.display_frame();
            Self::enforce_fps(timer);
            self.frame_counter += 1;
        }
    }

    fn enforce_fps(timer: SystemTime) {
        let sleep_duration = 1.0 / FPS_LIMIT;
        if let Ok(elapsed) = timer.elapsed() {
            if Duration::from_secs_f32(sleep_duration) > elapsed {
                sleep(Duration::from_secs_f32(sleep_duration) - elapsed);
            }
        } else {
            return;
        }
    }
}
