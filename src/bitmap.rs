use std::process::exit;

use crate::utils::{ANSI, XY};

#[derive(Clone)]
pub struct Bitmap<T> {
    pub resolution: XY<usize>,
    pub map: Vec<Vec<T>>,
}

impl<T: Clone> Bitmap<T> {
    pub fn new(resolution: XY<usize>, default_contents: T) -> Self {
        Bitmap {
            resolution,
            map: vec![vec![default_contents.clone(); resolution.x]; resolution.y],
        }
    }
}

pub struct BitmapRenderer;
impl BitmapRenderer {
    pub fn print_bitmap(bitmap: &Bitmap<char>, border_width: &XY<usize>) {
        for y in 0..bitmap.resolution.y {
            for x in 0..bitmap.resolution.x {
                print!(
                    "{}[{};{}f{}",
                    ANSI,
                    y + border_width.y,
                    x + border_width.x,
                    bitmap.map[y][x]
                );
            }
        }
    }
}

#[derive(Clone)]
pub struct BitmapBuffer {
    active_frame: Bitmap<char>,
    following_frame: Bitmap<char>,
    changed_pixels: Bitmap<bool>,
    resolution: XY<usize>,
}
impl BitmapBuffer {
    pub fn new(default_frame: &Bitmap<char>) -> Self {
        let resolution = default_frame.resolution;
        BitmapBuffer {
            active_frame: default_frame.clone(),
            following_frame: default_frame.clone(),
            changed_pixels: Bitmap::new(resolution, false),
            resolution,
        }
    }

    pub fn update(&mut self) {
        for col in 0..self.resolution.y {
            for row in 0..self.resolution.x {
                let new = self.following_frame.map[col][row];
                let old = self.active_frame.map[col][row];
                if old != new {
                    self.active_frame.map[col][row] = new;
                    self.changed_pixels.map[col][row] = true;
                } else {
                    self.changed_pixels.map[col][row] = false;
                }
            }
        }
    }

    pub fn new_following_frame(&mut self, new_frame: &Bitmap<char>) {
        if new_frame.resolution != self.resolution {
            exit(1);
        }
        self.following_frame = new_frame.clone();
    }

    pub fn get_active_frame(&self) -> &Bitmap<char> {
        &self.active_frame
    }
}
