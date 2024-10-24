use std::process::exit;

use crate::{asset_server::TRANSPARENT_CHAR, utils::{ESC, XY}, BORDER_WIDTH};

#[derive(Clone)]
pub struct Bitmap<T> {
    pub resolution: XY<usize>,
    pub matrix: Vec<Vec<T>>,
}

impl<T: Clone> Bitmap<T> {
    pub fn new(resolution: XY<usize>, default_contents: T) -> Self {
        Bitmap {
            resolution,
            matrix: vec![vec![default_contents.clone(); resolution.y]; resolution.x],
        }
    }
}

pub struct BitmapPrinter;
impl BitmapPrinter {
    pub fn print_bitmap(bitmap: &Bitmap<char>) {
        for row in 0..bitmap.resolution.x { // Iterate over rows first (x-axis)
            for col in 0..bitmap.resolution.y { // Iterate over columns (y-axis)
                if bitmap.matrix[col][row] == TRANSPARENT_CHAR {
                    continue;
                }
                print!(
                    "{}[{};{}f{}",
                    ESC,
                    row + BORDER_WIDTH.x, // Use row for x
                    col + BORDER_WIDTH.y, // Use col for y
                    bitmap.matrix[col][row]
                );
            }
        }
    }
}


#[derive(Clone)]
pub struct BitmapBuffer {
    pub active_frame: Bitmap<char>,
    pub following_frame: Bitmap<char>,
    pub resolution: XY<usize>,
}
impl BitmapBuffer {
    pub fn new(default_frame: &Bitmap<char>) -> Self {
        let resolution = default_frame.resolution;
        BitmapBuffer {
            active_frame: default_frame.clone(),
            following_frame: default_frame.clone(),
            resolution,
        }
    }

    pub fn update(&mut self) {
        for col in 0..self.resolution.y {
            for row in 0..self.resolution.x {
                let new = self.following_frame.matrix[col][row];
                let old = self.active_frame.matrix[col][row];
                if old != new {
                    self.active_frame.matrix[col][row] = new;
                } else {
                    self.active_frame.matrix[col][row] = TRANSPARENT_CHAR;
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
