use crate::{
    asset_server::TRANSPARENT_CHAR,
    drawable_object::{Drawable, DrawableObject, Label},
    utils::XY,
};

use super::bitmap::Bitmap;

pub struct FrameAssembler {
    frame: Bitmap<char>,
}
impl FrameAssembler {
    pub fn new(frame_resolution: XY<usize>, background: char) -> Self {
        FrameAssembler {
            frame: Bitmap::new(frame_resolution, background),
        }
    }

    pub fn new_default(frame_resolution: XY<usize>) -> Self {
        Self::new(frame_resolution, TRANSPARENT_CHAR)
    }

    // places a sprite on a bitmap by upper left corner of the sprite
    pub fn insert(&mut self, drawable_object: &mut DrawableObject, position: &XY<i32>) {
        let bitmap: &Bitmap<char> = drawable_object.get_bitmap();
        for row in 0..bitmap.resolution.x {
            for col in 0..bitmap.resolution.y {
                let target_x = position.x + row as i32;
                let target_y = position.y + col as i32;

                if target_x >= 0
                    && target_y >= 0
                    && (target_x as usize) < self.frame.resolution.x
                    && (target_y as usize) < self.frame.resolution.y
                {
                    if bitmap.matrix[col][row] == TRANSPARENT_CHAR {
                        continue;
                    }
                    self.frame.matrix[target_y as usize][target_x as usize] =
                        bitmap.matrix[col][row];
                }
            }
        }
    }

    pub fn insert_mut(&mut self, drawable_object: &mut DrawableObject, position: &XY<i32>) {
        let bitmap = drawable_object.get_bitmap();
        for row in 0..bitmap.resolution.x {
            for col in 0..bitmap.resolution.y {
                let target_x = position.x + row as i32;
                let target_y = position.y + col as i32;

                if target_x >= 0
                    && target_y >= 0
                    && (target_x as usize) < self.frame.resolution.x
                    && (target_y as usize) < self.frame.resolution.y
                {
                    self.frame.matrix[target_y as usize][target_x as usize] =
                        bitmap.matrix[col][row];
                }
            }
        }
    }

    pub fn get_frame(self) -> Box<Bitmap<char>> {
        Box::new(self.frame)
    }
}
impl FrameAssembler {
    pub fn label(&mut self, text: &str, position: (i32, i32)) {
        let mut label = DrawableObject::Label(Label::new(text));
        self.insert(&mut label, &XY::new(position.0, position.1));
    }
}
