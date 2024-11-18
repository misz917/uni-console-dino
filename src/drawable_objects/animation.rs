use super::drawable_object::Drawable;
use crate::bitmap_utils::bitmap::Bitmap;
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

#[cfg(test)]
mod tests {
    use crate::utils::XY;

    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    fn create_test_bitmap(content: char, width: usize, height: usize) -> Bitmap<char> {
        Bitmap::new(
            XY {
                x: width,
                y: height,
            },
            content,
        )
    }

    #[test]
    fn test_animation_creation() {
        let frame1 = create_test_bitmap('A', 3, 3);
        let frame2 = create_test_bitmap('B', 3, 3);
        let frames = vec![frame1.clone(), frame2.clone()];
        let fps = 2.0;

        let animation = Animation::new(&frames, fps);

        assert_eq!(animation.frames.len(), 2);
        assert_eq!(animation.frames[0], frame1);
        assert_eq!(animation.frames[1], frame2);

        assert_eq!(animation.fps, fps);
    }

    #[test]
    fn test_animation_get_frame() {
        let frame1 = create_test_bitmap('A', 3, 3);
        let frame2 = create_test_bitmap('B', 3, 3);
        let frames = vec![frame1.clone(), frame2.clone()];
        let fps = 1.0;

        let animation = Animation::new(&frames, fps);

        assert_eq!(animation.get_frame(), &frame1);

        sleep(Duration::from_secs_f32(0.5));
        assert_eq!(animation.get_frame(), &frame1);

        sleep(Duration::from_secs_f32(0.5));
        assert_eq!(animation.get_frame(), &frame2);

        sleep(Duration::from_secs_f32(1.0));
        assert_eq!(animation.get_frame(), &frame1);
    }

    #[test]
    fn test_animation_reset_time() {
        let frame1 = create_test_bitmap('A', 3, 3);
        let frame2 = create_test_bitmap('B', 3, 3);
        let frames = vec![frame1.clone(), frame2.clone()];
        let fps = 1.0;

        let mut animation = Animation::new(&frames, fps);

        sleep(Duration::from_secs_f32(1.0));
        assert_eq!(animation.get_frame(), &frame2);

        animation.reset_time();

        assert_eq!(animation.get_frame(), &frame1);

        sleep(Duration::from_secs_f32(1.0));
        assert_eq!(animation.get_frame(), &frame2);
    }

    #[test]
    fn test_animation_with_empty_frames() {
        let frames: Vec<Bitmap<char>> = Vec::new();
        let fps = 1.0;

        let animation = Animation::new(&frames, fps);

        assert_eq!(animation.frames.len(), 0);
    }

    #[test]
    fn test_animation_get_bitmap_trait() {
        let frame1 = create_test_bitmap('X', 2, 2);
        let frame2 = create_test_bitmap('O', 2, 2);
        let frames = vec![frame1.clone(), frame2.clone()];
        let fps = 2.0;

        let animation = Animation::new(&frames, fps);

        assert_eq!(animation.get_bitmap(), &frame1);

        sleep(Duration::from_secs_f32(1.0));
        assert_eq!(animation.get_bitmap(), &frame1);

        sleep(Duration::from_secs_f32(1.0));
        assert_eq!(animation.get_bitmap(), &frame2);
    }
}
