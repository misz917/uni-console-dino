use crate::bitmap_utils::bitmap::Bitmap;
use std::time::SystemTime;

use super::drawable_object::Drawable;

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
        let current_frame_num =
            (self.time.elapsed().unwrap().as_secs_f32() / self.fps) as usize % self.frames.len();
        &self.frames[current_frame_num]
    }

    pub fn reset_time(&mut self) {
        self.time = SystemTime::now();
    }
}
impl Drawable for Animation {
    fn get_bitmap(&self) -> &Bitmap<char> {
        &self.get_frame()
    }
}