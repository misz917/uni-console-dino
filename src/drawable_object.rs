use crate::{
    bitmap::Bitmap,
    utils::XY,
};

pub trait ImmutableBitmapContainer<T> {
    fn get_bitmap(&self) -> &Bitmap<T>;
}

pub trait MutableBitmapContainer<T> {
    fn get_bitmap_mut(&mut self) -> &Bitmap<T>;
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
impl ImmutableBitmapContainer<char> for Label {
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
impl ImmutableBitmapContainer<char> for Sprite {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.0
    }
}

pub trait FrameUpdater {
    fn next_frame(&mut self) -> usize;
}

impl FrameUpdater for Animation {
    fn next_frame(&mut self) -> usize {
        self.active_frame = (self.active_frame + 1) % self.frames.len();
        self.active_frame
    }
}

#[derive(Clone)]
pub struct Animation {
    frames: Vec<Bitmap<char>>,
    active_frame: usize,
}
impl Animation {
    pub fn new(frames: &Vec<Bitmap<char>>) -> Self { // maybe could be optimised with Box
        Animation {
            frames: frames.clone(),
            active_frame: 0,
        }
    }

    fn update(&mut self) {
        self.next_frame();
    }

    fn get_active_frame(&self) -> &Bitmap<char> {
        &self.frames[self.active_frame]
    }
}
impl ImmutableBitmapContainer<char> for Animation {
    fn get_bitmap(&self) -> &Bitmap<char> {
        self.get_active_frame()
    }
}
impl MutableBitmapContainer<char> for Animation {
    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
        self.update();
        self.get_active_frame()
    }
}

#[derive(Clone)]
pub enum MutableDrawableObject {
    Animation(Animation),
}
impl MutableBitmapContainer<char> for MutableDrawableObject {
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
impl ImmutableBitmapContainer<char> for ImmutableDrawableObject {
    fn get_bitmap(&self) -> &Bitmap<char> {
        match self {
            ImmutableDrawableObject::Label(label) => label.get_bitmap(),
            ImmutableDrawableObject::Sprite(sprite) => sprite.get_bitmap(),
        }
    }
}

pub enum DrawableObject {
    Immutable(ImmutableDrawableObject),
    Mutable(MutableDrawableObject),
}