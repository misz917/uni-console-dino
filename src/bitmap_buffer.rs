use crate::{
    asset_server::TRANSPARENT_CHAR,
    bitmap::Bitmap,
    utils::{ErrorDisplayer, XY},
};

#[derive(Clone)]
pub struct BitmapBuffer {
    active_frame: Box<Bitmap<char>>,
    following_frame: Option<Box<Bitmap<char>>>,
    resolution: XY<usize>,
}
impl BitmapBuffer {
    pub fn new(first_frame: &Bitmap<char>) -> Self {
        BitmapBuffer {
            active_frame: Box::new(first_frame.clone()),
            following_frame: None,
            resolution: first_frame.resolution,
        }
    }
}

pub trait BufferManager {
    fn insert_frame(&mut self, frame: Box<Bitmap<char>>);
    fn get_frame(&mut self) -> &Bitmap<char>; // generates a frame of only differences between following and active
}

impl BufferManager for BitmapBuffer {
    fn insert_frame(&mut self, new_frame: Box<Bitmap<char>>) {
        if new_frame.resolution != self.resolution {
            ErrorDisplayer::error("New frame has incorrect resolution");
        }

        if let Some(bitmap) = self.following_frame.take() {
            self.active_frame = bitmap;
        }

        self.following_frame = Some(new_frame);
    }

    fn get_frame(&mut self) -> &Bitmap<char> {
        todo!()
        // self.active_frame = self.following_frame.clone();
        // &self.following_frame
    }

    // fn update(&mut self) {
    //     if !self.possible_update {
    //         return;
    //     }

    //     for row in 0..self.resolution.y {
    //         for col in 0..self.resolution.x {
    //             if self.active_frame.matrix[row][col] == self.following_frame.matrix[row][col] {
    //                 self.active_frame.matrix[row][col] = TRANSPARENT_CHAR.clone();
    //             } else {
    //                 self.active_frame.matrix[row][col] = self.following_frame.matrix[row][col].clone();
    //             }
    //         }
    //     }
        
    //     self.possible_update = false;
    // }
}