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
    fn get_frame(&mut self) -> Option<Box<Bitmap<char>>>;
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

    // generates a frame of transparent chars and differences between following and active
    fn get_frame(&mut self) -> Option<Box<Bitmap<char>>> {
        if let Some(following_frame) = &self.following_frame {
            let mut differences = Bitmap::new(self.resolution, TRANSPARENT_CHAR);
            for row in 0..self.resolution.y {
                for col in 0..self.resolution.x {
                    if self.active_frame.matrix[row][col] != following_frame.matrix[row][col] {
                        differences.matrix[row][col] = following_frame.matrix[row][col];
                    }
                }
            }
            return Some(Box::new(differences));
        }
        return None;
    }
}
