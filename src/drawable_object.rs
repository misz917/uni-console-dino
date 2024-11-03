use crate::{animation::Animation, bitmap::Bitmap, utils::XY};

pub trait Drawable {
    fn get_bitmap(&self) -> &Bitmap<char>;
}

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

#[derive(Clone, Debug)]
pub enum DrawableObject {
    Sprite(Sprite),
    Label(Label),
    Animation(Animation),
    Rectangle(Rectangle),
}
impl Drawable for DrawableObject {
    fn get_bitmap(&self) -> &Bitmap<char> {
        match self {
            DrawableObject::Sprite(sprite) => sprite.get_bitmap(),
            DrawableObject::Label(label) => label.get_bitmap(),
            DrawableObject::Animation(animation) => animation.get_bitmap(),
            DrawableObject::Rectangle(rectangle) => rectangle.get_bitmap(),
        }
    }
}
