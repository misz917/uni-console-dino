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
                bitmap.matrix[row][col] = sprite.bitmap.matrix[row][col];
            }
        }
    }
}
