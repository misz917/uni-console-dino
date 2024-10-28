use crate::{
    asset_server::TRANSPARENT_CHAR,
    bitmap::Bitmap,
    drawable_object::{Drawable, DrawableObject},
    utils::XY
};

pub struct FrameAssembler {
    frame: Bitmap<char>,
}
impl FrameAssembler {
    pub fn new(frame_resolution: XY<usize>) -> Self {
        FrameAssembler {
            frame: Bitmap::new(frame_resolution, TRANSPARENT_CHAR)
        }
    }

    // places a sprite on a bitmap by upper left corner of the sprite
    pub fn insert(&mut self, drawable_object: &DrawableObject, position: &XY<i32>) {
        let bitmap = drawable_object.get_bitmap();
        for row in 0..bitmap.resolution.x {
            for col in 0..bitmap.resolution.y {
                let target_x = position.x + row as i32;
                let target_y = position.y + col as i32;
    
                if target_x >= 0 && target_y >= 0
                    && (target_x as usize) < self.frame.resolution.x
                    && (target_y as usize) < self.frame.resolution.y
                {
                    self.frame.matrix[target_y as usize][target_x as usize] = bitmap.matrix[col][row];
                }
            }
        }
    }

    pub fn insert_mut(&mut self, drawable_object: &mut DrawableObject, position: &XY<i32>) {
        let bitmap = drawable_object.get_bitmap_mut();
        for row in 0..bitmap.resolution.x {
            for col in 0..bitmap.resolution.y {
                let target_x = position.x + row as i32;
                let target_y = position.y + col as i32;
    
                if target_x >= 0 && target_y >= 0
                    && (target_x as usize) < self.frame.resolution.x
                    && (target_y as usize) < self.frame.resolution.y
                {
                    self.frame.matrix[target_y as usize][target_x as usize] = bitmap.matrix[col][row];
                }
            }
        }
    }
    
    pub fn get_frame(self) -> Box<Bitmap<char>> {
        Box::new(self.frame)
    }
}
