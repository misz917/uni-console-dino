use std::process::exit;

use crate::{asset_server::TRANSPARENT_CHAR, utils::{ErrorDisplayer, ESC, XY}, BORDER_WIDTH};

#[derive(Clone)]
pub struct Bitmap<T> {
    pub resolution: XY<usize>,
    pub matrix: Vec<Vec<T>>,
}

impl<T: Clone> Bitmap<T> {
    pub fn new(resolution: XY<usize>, default_contents: T) -> Self {
        Bitmap {
            resolution,
            matrix: vec![vec![default_contents.clone(); resolution.x]; resolution.y],
        }
    }
}


pub struct BitmapPrinter;
impl BitmapPrinter {
    pub fn print_bitmap (bitmap: &Bitmap<char>, border_width: &XY<usize>) {
        for (i, row) in bitmap.matrix.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == TRANSPARENT_CHAR {
                    continue;
                }
                print!("{}[{};{}H{}", ESC, i + 1 + border_width.y, j + 1 + border_width.x, item);
            }
        }
    }
}


#[derive(Clone)]
pub struct BitmapBuffer {
    active_frame: Bitmap<char>,
    following_frame: Bitmap<char>,
    possible_update: bool,
    resolution: XY<usize>,
}
impl BitmapBuffer {
    pub fn new(default_frame: &Bitmap<char>) -> Self {
        let resolution = default_frame.resolution;
        BitmapBuffer {
            active_frame: default_frame.clone(),
            following_frame: default_frame.clone(),
            possible_update: true,
            resolution,
        }
    }

    pub fn update(&mut self) {
        if !self.possible_update {
            return;
        }
        for row in 0..self.resolution.y {
            for col in 0..self.resolution.x {
                if self.active_frame.matrix[row][col] == self.following_frame.matrix[row][col] {
                    self.active_frame.matrix[row][col] = TRANSPARENT_CHAR.clone();
                } else {
                    self.active_frame.matrix[row][col] = self.following_frame.matrix[row][col].clone();
                }
            }
        }
        self.possible_update = false;
    }

    pub fn new_following_frame(&mut self, new_frame: &Bitmap<char>) {
        if new_frame.resolution != self.resolution {
            ErrorDisplayer::error("New frame has incorrect resolution");
        }
        self.following_frame = new_frame.clone();
        self.possible_update = true;
    }

    pub fn get_active_frame(&self) -> &Bitmap<char> {
        &self.active_frame
    }
}
