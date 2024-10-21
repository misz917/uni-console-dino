use std::{env, fs::read, process::Command};

#[derive(Clone, Copy, Debug)]
struct XY<T> {
    x: T,
    y: T,
}

impl<T> XY<T> {
    const fn new(x: T, y: T) -> Self {
        XY { x, y }
    }
}

const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 45);
//160x90 but x axis is 2 times denser

struct Bitmap<T> {
    resolution: XY<usize>,
    map: Vec<Vec<T>>,
}

impl<T: Clone> Bitmap<T> {
    fn new(resolution: XY<usize>, default_contents: T) -> Self {
        Bitmap {
            resolution: resolution,
            map: vec![vec![default_contents.clone(); resolution.x]; resolution.y],
        }
    }
}

struct BitmapBuffer {
    active_frame: Bitmap<char>,
    following_frame: Bitmap<char>,
}

struct AssetServer;

struct BitmapRenderer;
impl BitmapRenderer {
    fn print_bitmap(bitmap: &Bitmap<char>) {
        for y in 0..bitmap.resolution.y {
            for x in 0..bitmap.resolution.x {
                print!("{}", bitmap.map[y][x]);
            }
            print!("\n");
        }
    }
}

struct WindowCreator;
impl WindowCreator {
    fn open_new_window(resolution: XY<usize>) {
        let output = Command::new("gnome-terminal")
        .args(&[
            "--geometry", &format!("{}x{}", resolution.x, resolution.y),
            "--", "bash", "-c", &format!("{} -ready", env::current_exe().unwrap().to_string_lossy())
        ])
        .output()
        .expect("Failure");

        if output.status.success() {
            println!("Terminal opened successfully.");
        } else {
            eprintln!("Failed to open terminal.");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ready: bool = false;
    for arg in &args {
        if arg == "-ready" {
            ready = true;
        }
    }
    
    if !ready {
        WindowCreator::open_new_window(WINDOW_RESOLUTION);
        return;
    }
    
    println!("Hello world");
    loop {}
    // let bitmap = Bitmap::new(WINDOW_RESOLUTION, '#');
    // BitmapRenderer::print_bitmap(&bitmap);
    // WindowCreator::open_new_window(WINDOW_RESOLUTION);
    // println!("{}", env::current_exe().unwrap().to_string_lossy());
}
