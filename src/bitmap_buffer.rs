use crate::{
    asset_server::TRANSPARENT_CHAR,
    bitmap::Bitmap,
    utils::{ErrorDisplayer, XY},
};

pub trait BufferManager {
    fn new_following_frame(&mut self, frame: &Bitmap<char>);
    fn get_active_frame(&self) -> &Bitmap<char>;
    fn update(&mut self);
}

#[derive(Clone)]
pub struct BitmapBuffer {
    active_frame: Bitmap<char>,
    following_frame: Bitmap<char>,
    resolution: XY<usize>,
    possible_update: bool,
}
impl BitmapBuffer {
    pub fn new(default_frame: &Bitmap<char>) -> Self {
        BitmapBuffer {
            active_frame: default_frame.clone(),
            following_frame: default_frame.clone(),
            resolution: default_frame.resolution,
            possible_update: true,
        }
    }
}
impl BufferManager for BitmapBuffer {
    fn new_following_frame(&mut self, new_frame: &Bitmap<char>) {
        if new_frame.resolution != self.resolution {
            ErrorDisplayer::error("New frame has incorrect resolution");
        }
        self.following_frame = new_frame.clone();
        self.possible_update = true;
    }

    fn get_active_frame(&self) -> &Bitmap<char> {
        &self.active_frame
    }

    fn update(&mut self) {
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
}