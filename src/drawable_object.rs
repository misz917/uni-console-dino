use crate::{
    bitmap::Bitmap,
    utils::XY,
};

pub trait Drawable {
    fn get_bitmap(&self) -> &Bitmap<char>;
    fn get_bitmap_mut(&mut self) -> &Bitmap<char>;
}

#[derive(Clone)]
pub struct Sprite(Bitmap<char>);
impl Sprite {
    pub fn new(bitmap: &Bitmap<char>) -> Self {
        Sprite( bitmap.clone() )
    }
}
impl Drawable for Sprite {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.0
    }

    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
        &self.0
    }
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
impl Drawable for Label {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.0
    }

    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
        &self.0
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
impl Drawable for Animation {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.get_active_frame()
    }

    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
        self.update();
        self.get_active_frame()
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
pub enum DrawableObject {
    Sprite(Sprite),
    Label(Label),
    Animation(Animation),
}
impl Drawable for DrawableObject {
    fn get_bitmap(&self) -> &Bitmap<char> {
        match self {
            DrawableObject::Sprite(sprite) => sprite.get_bitmap(),
            DrawableObject::Label(label) => label.get_bitmap(),
            DrawableObject::Animation(animation) => animation.get_bitmap(),
        }
    }

    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
        match self {
            DrawableObject::Sprite(sprite) => sprite.get_bitmap_mut(),
            DrawableObject::Label(label) => label.get_bitmap_mut(),
            DrawableObject::Animation(animation) => animation.get_bitmap_mut(),
        }
    }
}