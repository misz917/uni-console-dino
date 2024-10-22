use std::{env, process::Command};
use crate::utils::XY;

pub trait Terminal {
    fn open(&self, resolution: XY<usize>, border_width: usize);
}

pub struct GnomeTerminal;
impl Terminal for GnomeTerminal {
    fn open(&self, resolution: XY<usize>, border_width: usize) {
        let output = Command::new("gnome-terminal")
            .args(&[
                "--geometry",
                &format!("{}x{}", resolution.x + border_width * 2, resolution.y + border_width * 2),
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
    pub fn open_new_window<T: Terminal>(terminal: T, resolution: XY<usize>, border_width: usize) {
        terminal.open(resolution, border_width);
    }
}
