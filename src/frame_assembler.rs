use crate::{
    bitmap::Bitmap,
    utils::{Sprite, XY},
};

pub struct FrameAssembler;
impl FrameAssembler {
    // places a sprite on a bitmap by upper left corner of the sprite
    pub fn write_sprite_to_bitmap(sprite: &Sprite, bitmap: &mut Bitmap<char>, position: &XY<i32>) {
        for row in 0..sprite.bitmap.resolution.x {
            for col in 0..sprite.bitmap.resolution.y {
                let target_x = position.x + row as i32;
                let target_y = position.y + col as i32;
    
                if target_x >= 0 && target_y >= 0
                    && (target_x as usize) < bitmap.resolution.x
                    && (target_y as usize) < bitmap.resolution.y
                {
                    bitmap.matrix[target_y as usize][target_x as usize] = sprite.bitmap.matrix[col][row];
                }
            }
        }
    }
    
}
