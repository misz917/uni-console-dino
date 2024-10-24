use crate::{
    asset_server::TRANSPARENT_CHAR, bitmap::Bitmap, utils::{Sprite, XY}
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
    pub fn insert_sprite(&mut self, sprite: &Sprite, position: &XY<i32>) {
        for row in 0..sprite.bitmap.resolution.x {
            for col in 0..sprite.bitmap.resolution.y {
                let target_x = position.x + row as i32;
                let target_y = position.y + col as i32;
    
                if target_x >= 0 && target_y >= 0
                    && (target_x as usize) < self.frame.resolution.x
                    && (target_y as usize) < self.frame.resolution.y
                {
                    self.frame.matrix[target_y as usize][target_x as usize] = sprite.bitmap.matrix[col][row];
                }
            }
        }
    }
    
    pub fn get_frame(self) -> Box<Bitmap<char>> {
        Box::new(self.frame)
    }
}
