use crate::{
    bitmap::Bitmap,
    utils::{Sprite, XY},
};

pub struct FrameAssembler;
impl FrameAssembler {
    // places a sprite on a bitmap by upper left corner of the sprite
    pub fn write_sprite_to_bitmap(sprite: &Sprite, bitmap: &mut Bitmap<char>, position: &XY<i32>) {
        for col in 0..sprite.bitmap.resolution.x {
            for row in 0..sprite.bitmap.resolution.y {
                if row as i32 + position.y < 0 || col as i32 + position.x < 0 {
                    continue;
                }
                if row as i32 + position.y >= bitmap.resolution.y as i32
                    || col as i32 + position.x >= bitmap.resolution.x as i32
                {
                    continue;
                }

                bitmap.matrix[row + position.y as usize][col + position.x as usize] =
                    sprite.bitmap.matrix[row][col];
            }
        }
    }
}
