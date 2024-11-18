use super::bitmap::Bitmap;
use crate::{
    asset_server::TRANSPARENT_CHAR,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_bitmap(content: char, width: usize, height: usize) -> Bitmap<char> {
        Bitmap::new(
            XY {
                x: width,
                y: height,
            },
            content,
        )
    }

    #[test]
    fn test_bitmap_buffer_creation() {
        let first_frame = create_test_bitmap('A', 3, 3);
        let buffer = BitmapBuffer::new(&first_frame);

        assert_eq!(buffer.resolution.x, 3);
        assert_eq!(buffer.resolution.y, 3);

        assert_eq!(buffer.active_frame.matrix[0][0], 'A');
    }

    #[test]
    fn test_insert_frame_with_correct_resolution() {
        let first_frame = create_test_bitmap('A', 3, 3);
        let mut buffer = BitmapBuffer::new(&first_frame);

        let second_frame = create_test_bitmap('B', 3, 3);

        buffer.insert_frame(Box::new(second_frame.clone()));

        assert_eq!(buffer.active_frame.matrix[0][0], 'A');
        assert_eq!(buffer.following_frame.as_ref().unwrap().matrix[0][0], 'B');
    }

    #[test]
    fn test_get_frame_no_difference() {
        let first_frame = create_test_bitmap('A', 3, 3);
        let second_frame = create_test_bitmap('A', 3, 3);
        let mut buffer = BitmapBuffer::new(&first_frame);

        buffer.insert_frame(Box::new(second_frame));

        let difference_frame = buffer.get_frame();

        let difference_frame = difference_frame.unwrap();
        for row in difference_frame.matrix {
            for cell in row {
                assert_eq!(cell, TRANSPARENT_CHAR);
            }
        }
    }

    #[test]
    fn test_get_frame_with_differences() {
        let first_frame = create_test_bitmap('A', 3, 3);
        let second_frame = create_test_bitmap('B', 3, 3);
        let mut buffer = BitmapBuffer::new(&first_frame);

        // Insert the second frame into the buffer
        buffer.insert_frame(Box::new(second_frame));

        // Get the difference frame
        let difference_frame = buffer.get_frame();

        // Verify that the difference frame has the new character 'B' where there is a change
        let difference_frame = difference_frame.unwrap();
        assert_eq!(difference_frame.matrix[0][0], 'B');
        assert_eq!(difference_frame.matrix[1][1], 'B');
        assert_eq!(difference_frame.matrix[2][2], 'B');
        // The rest should be TRANSPARENT_CHAR
        assert_eq!(difference_frame.matrix[0][1], TRANSPARENT_CHAR);
        assert_eq!(difference_frame.matrix[0][2], TRANSPARENT_CHAR);
        assert_eq!(difference_frame.matrix[1][0], TRANSPARENT_CHAR);
        assert_eq!(difference_frame.matrix[2][0], TRANSPARENT_CHAR);
        assert_eq!(difference_frame.matrix[2][1], TRANSPARENT_CHAR);
    }

    #[test]
    fn test_get_frame_no_following_frame() {
        let first_frame = create_test_bitmap('A', 3, 3);
        let mut buffer = BitmapBuffer::new(&first_frame);

        let difference_frame = buffer.get_frame();

        assert!(difference_frame.is_none());
    }
}
