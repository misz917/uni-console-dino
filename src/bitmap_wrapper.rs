use crate::bitmap::Bitmap;

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