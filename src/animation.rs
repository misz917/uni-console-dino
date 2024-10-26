use crate::bitmap::Bitmap;

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