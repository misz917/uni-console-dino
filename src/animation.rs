use crate::{bitmap::Bitmap, drawable_object::Drawable};
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct Animation {
    frames: Vec<Bitmap<char>>,
    time: SystemTime,
    fps: f32,
}
impl Animation {
    pub fn new(frames: &Vec<Bitmap<char>>, fps: f32) -> Self {
        Animation {
            frames: frames.clone(),
            time: SystemTime::now(),
            fps,
        }
    }

    fn get_frame(&self) -> &Bitmap<char> {
        &self.frames[0]
    }
}
impl Drawable for Animation {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.get_frame()
    }

    fn get_bitmap_mut(&mut self) -> &Bitmap<char> {
        self.get_frame()
    }
}
