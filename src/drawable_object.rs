use crate::{
    bitmap::Bitmap,
    utils::XY,
};

pub trait BitmapContainer {
    fn get_bitmap(&self) -> &Bitmap<char>;
}

pub trait MutableBitmapContainer {
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

#[derive(Clone)]
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

    fn update(&mut self) {
        self.active_frame = (self.active_frame + 1) % self.number_of_frames;
    }

    fn get_active_frame(&mut self) -> &Bitmap<char> {
        return &self.frames[self.active_frame];
    }
}
impl MutableBitmapContainer for Animation {
    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
        self.update();
        return self.get_active_frame();
    }
}

#[derive(Clone)]
pub enum MutableDrawableObject {
    Animation(Animation),
}
impl MutableBitmapContainer for MutableDrawableObject {
    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
        match self {
            MutableDrawableObject::Animation(animation) => animation.get_bitmap_mut(),
        }
    }
}

pub enum ImmutableDrawableObject {
    Label(Label),
    Sprite(Sprite),
}
impl BitmapContainer for ImmutableDrawableObject {
    fn get_bitmap(&self) -> &Bitmap<char> {
        match self {
            ImmutableDrawableObject::Label(label) => label.get_bitmap(),
            ImmutableDrawableObject::Sprite(sprite) => sprite.get_bitmap(),
        }
    }
}