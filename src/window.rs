use crate::{utils::XY, BORDER_WIDTH, WINDOW_RESOLUTION};
use std::{env, process::{exit, Command}};

pub enum TerminalType {
    GnomeTerminal,
}

pub trait Terminal {
    fn open(&self, resolution: XY<usize>, border_width: XY<usize>);
}

pub struct GnomeTerminal;
impl Terminal for GnomeTerminal {
    fn open(&self, resolution: XY<usize>, border_width: XY<usize>) {
        let output = Command::new("gnome-terminal")
            .args(&[
                "--geometry",
                &format!(
                    "{}x{}",
                    resolution.x + border_width.x * 2,
                    resolution.y + border_width.y * 2,
                ),
                "--",
                "bash",
                "-c",
                &format!("{} -ready", env::current_exe().unwrap().to_string_lossy()),
            ])
            .output()
            .expect("Failed to open");

        TerminalResultHandler::handle(output);
    }
}

struct TerminalResultHandler;
impl TerminalResultHandler {
    fn handle(output: std::process::Output) {
        if output.status.success() {
            println!("Terminal opened successfully.");
        } else {
            eprintln!("Failed to open terminal.");
        }
    }
}

pub struct WindowCreator;
impl WindowCreator {
    fn open_new_window<T: Terminal>(
        terminal: &T,
        resolution: XY<usize>,
        border_width: XY<usize>,
    ) {
        terminal.open(resolution, border_width);
    }

    pub fn separate_window_creation<T: Terminal>(window_resolution: XY<usize>, border_width: XY<usize>, terminal: &T) {
        let args: Vec<String> = env::args().collect();
        let mut ready: bool = false;
        for arg in &args {
            if arg == "-ready" {
                ready = true;
            }
        }
        if !ready {
            Self::open_new_window(
                terminal,
                window_resolution,
                border_width,
            );
            exit(0); // this exit is not an error
        }
    }
}
