use crate::utils::XY;
use std::{env, io::Read, os::fd::AsRawFd, process::{exit, Command}};

pub trait Terminal {
    fn open(&self, resolution: XY<usize>, border_width: XY<usize>);
}

pub struct UnixTerminalHandler;
impl UnixTerminalHandler {
    // C wizardry, stolen from the internet
    pub fn set_raw_mode() {
        let fd = std::io::stdin().as_raw_fd();
        let mut old_termios = libc::termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
            c_line: 0,
        };
        unsafe {
            libc::tcgetattr(fd, &mut old_termios);
            let mut raw = old_termios;
            raw.c_lflag &= !(libc::ICANON | libc::ECHO);
            raw.c_cc[libc::VMIN] = 1;
            raw.c_cc[libc::VTIME] = 0;
            libc::tcsetattr(fd, libc::TCSANOW, &raw);
        }
    }

    pub fn read_key() -> Option<char> {
        let mut buffer = [0; 1];
        if let Ok(n) = std::io::stdin().read(&mut buffer) {
            if n > 0 {
                return Some(buffer[0] as char);
            }
        }
        None
    }
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

    pub fn create_separate_window<T: Terminal>(window_resolution: XY<usize>, border_width: XY<usize>, terminal: &T) {
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
