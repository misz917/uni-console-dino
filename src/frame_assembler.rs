use crate::{
    bitmap::Bitmap,
    utils::{Sprite, XY},
};

pub struct FrameAssembler;
impl FrameAssembler {
    // places a sprite on a bitmap by upper left corner of the sprite
    pub fn write_sprite_to_bitmap(sprite: &Sprite, bitmap: &mut Bitmap<char>, position: &XY<i32>) {
        for x in 0..sprite.bitmap.resolution.x {
            for y in 0..sprite.bitmap.resolution.y {
                bitmap.matrix[x + position.x as usize][y + position.y as usize] = sprite.bitmap.matrix[x][y];
            }
        }
    }

    fn is_within_bounds(position: &XY<i32>, to_add: (usize, usize), bounds: &XY<usize>) -> bool {
        if position.x + to_add.0 as i32 >= 0
            && position.y + to_add.1 as i32 >= 0
            && bounds.x as i32 > position.x + to_add.0 as i32
            && bounds.y as i32 > position.y + to_add.1 as i32
        {
            return true;
        }
        return false;
    }
}
