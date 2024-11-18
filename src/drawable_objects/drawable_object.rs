use super::{animation::Animation, label::Label, rectangle::Rectangle, sprite::Sprite};
use crate::bitmap_utils::bitmap::Bitmap;

pub trait Drawable {
    fn get_bitmap(&self) -> &Bitmap<char>;
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
