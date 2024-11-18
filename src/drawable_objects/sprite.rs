use crate::bitmap_utils::bitmap::Bitmap;

use super::drawable_object::Drawable;

#[derive(Clone, Debug)]
pub struct Sprite(Bitmap<char>);
impl Sprite {
    pub fn new(bitmap: &Bitmap<char>) -> Self {
        Sprite(bitmap.clone())
    }
}
impl Drawable for Sprite {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.0
    }
}
