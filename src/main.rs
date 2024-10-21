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

const WINDOW_RESOLUTION: XY<usize> = XY::new(80, 45);

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

struct ScreenBuffer {
    active_frame: Vec<Vec<char>>,
    following_frame: Vec<Vec<char>>,
}

struct AssetServer;

struct BitmapRenderer;

fn main() {
    println!("Hello world!");
}
