use std::process::exit;

use crate::utils::{ESC, XY};

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

pub struct BufferPrinter;
impl BufferPrinter {
    pub fn print_bitmap(bitmap_buffer: &BitmapBuffer, border_width: &XY<usize>) {
        for y in 0..bitmap_buffer.resolution.y {
            for x in 0..bitmap_buffer.resolution.x {
                if bitmap_buffer.get_active_frame().matrix[y][x] == '$' || !bitmap_buffer.changed_pixels.matrix[y][x] {
                    continue;
                }
                print!(
                    "{}[{};{}f{}",
                    ESC,
                    y + border_width.y,
                    x + border_width.x,
                    bitmap_buffer.active_frame.matrix[y][x]
                );
            }
        }
    }
}

#[derive(Clone)]
pub struct BitmapBuffer {
    pub active_frame: Bitmap<char>,
    pub following_frame: Bitmap<char>,
    pub changed_pixels: Bitmap<bool>,
    pub resolution: XY<usize>,
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
                let new = self.following_frame.matrix[col][row];
                let old = self.active_frame.matrix[col][row];
                if old != new {
                    self.active_frame.matrix[col][row] = new;
                    self.changed_pixels.matrix[col][row] = true;
                } else {
                    self.changed_pixels.matrix[col][row] = false;
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
