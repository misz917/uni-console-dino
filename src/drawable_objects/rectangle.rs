use crate::{bitmap_utils::bitmap::Bitmap, utils::XY};

use super::drawable_object::Drawable;

#[derive(Clone, Debug)]
pub struct Rectangle(Bitmap<char>);
impl Rectangle {
    pub fn new(dimensions: XY<usize>, filling: char) -> Self {
        Rectangle(Bitmap::new(dimensions, filling))
    }
}
impl Drawable for Rectangle {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.0
    }
}
