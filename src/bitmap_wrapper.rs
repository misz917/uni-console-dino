use crate::{bitmap::Bitmap, utils::XY};

pub trait ExtendsBitmap {

}

#[derive(Clone)]
pub struct Sprite {
    pub bitmap: Bitmap<char>,
}
impl Sprite {
    pub fn from_bitmap(bitmap: &Bitmap<char>) -> Self {
        Sprite {
            bitmap: bitmap.clone()
        }
    }
}

impl Bitmap<char> {
    pub fn from_string(text: &str) -> Self {
        let matrix: Vec<Vec<char>> = vec![text.chars().collect()];
        Bitmap {
            resolution: XY::new(text.len(), 1),
            matrix,
        }
    }
}