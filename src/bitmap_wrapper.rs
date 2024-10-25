use crate::{
    bitmap::Bitmap,
    utils::XY,
};

pub trait BitmapContainer {
    fn get_bitmap(&self) -> &Bitmap<char>;
}

#[derive(Clone)]
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
impl BitmapContainer for Label {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.0
    }
}

#[derive(Clone)]
pub struct Sprite(Bitmap<char>);
impl Sprite {
    pub fn new(bitmap: &Bitmap<char>) -> Self {
        Sprite( bitmap.clone() )
    }
}
impl BitmapContainer for Sprite {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.0
    }
}
