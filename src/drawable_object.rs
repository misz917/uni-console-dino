use crate::{
    bitmap::Bitmap,
    utils::XY,
};

pub trait BitmapContainer {
    fn get_bitmap(&self) -> &Bitmap<char>;
    fn get_bitmap_mut(&mut self) -> &Bitmap<char>;
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
    
    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
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

    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
        &self.0
    }
}

pub struct Animation {
    frames: Vec<Bitmap<char>>,
    number_of_frames: usize,
    active_frame: usize,
}
impl Animation {
    pub fn new(frames: &Vec<Bitmap<char>>) -> Self { // maybe could be optimised with Box
        Animation {
            frames: frames.clone(),
            number_of_frames: frames.len(),
            active_frame: frames.len() - 1,
        }
    }

    pub fn get_frame(&mut self) -> &Bitmap<char> {
        self.active_frame = (self.active_frame + 1) % self.number_of_frames;
        return &self.frames[self.active_frame];
    }
}

pub enum DrawableObject {
    Animation(Animation),
    Label(Label),
    Sprite(Sprite),
}
impl BitmapContainer for DrawableObject {
    fn get_bitmap(&self) -> &Bitmap<char> {
        match self {
            DrawableObject::Animation(animation) => &animation.frames[animation.active_frame],
            DrawableObject::Label(label) => label.get_bitmap(),
            DrawableObject::Sprite(sprite) => sprite.get_bitmap(),
        }
    }
    
    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
        match self {
            DrawableObject::Animation(animation) => animation.get_frame(),
            DrawableObject::Label(label) => label.get_bitmap_mut(),
            DrawableObject::Sprite(sprite) => sprite.get_bitmap_mut(),
        }
    }
}