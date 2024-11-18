use crate::{bitmap_utils::bitmap::Bitmap, utils::XY};

use super::drawable_object::Drawable;

#[derive(Clone, Debug)]
pub struct Label(Bitmap<char>);
impl Label {
    pub fn new(text: &str) -> Self {
        let matrix: Vec<Vec<char>> = vec![text.chars().collect()];
        Label(Bitmap {
            resolution: XY::new(text.len(), 1),
            matrix,
        })
    }
}
impl Drawable for Label {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.0
    }
}
