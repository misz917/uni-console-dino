use crate::{
    asset_server::TRANSPARENT_CHAR,
    drawable_objects::{
        drawable_object::{Drawable, DrawableObject},
        label::Label,
    },
    utils::XY,
};
use super::bitmap::Bitmap;

pub struct FrameAssembler {
    frame: Bitmap<char>,
}
impl FrameAssembler {
    pub fn new(frame_resolution: XY<usize>, background: char) -> Self {
        FrameAssembler {
            frame: Bitmap::new(frame_resolution, background),
        }
    }

    pub fn new_default(frame_resolution: XY<usize>) -> Self {
        Self::new(frame_resolution, TRANSPARENT_CHAR)
    }

    // places a sprite on a bitmap by upper left corner of the sprite
    pub fn insert(&mut self, drawable_object: &mut DrawableObject, position: &XY<i32>) {
        let bitmap: &Bitmap<char> = drawable_object.get_bitmap();
        for row in 0..bitmap.resolution.x {
            for col in 0..bitmap.resolution.y {
                let target_x = position.x + row as i32;
                let target_y = position.y + col as i32;

                if target_x >= 0
                    && target_y >= 0
                    && (target_x as usize) < self.frame.resolution.x
                    && (target_y as usize) < self.frame.resolution.y
                {
                    if bitmap.matrix[col][row] == TRANSPARENT_CHAR {
                        continue;
                    }
                    self.frame.matrix[target_y as usize][target_x as usize] =
                        bitmap.matrix[col][row];
                }
            }
        }
    }

    pub fn get_frame(self) -> Box<Bitmap<char>> {
        Box::new(self.frame)
    }
}
impl FrameAssembler {
    pub fn label(&mut self, text: &str, position: (i32, i32)) {
        let mut label = DrawableObject::Label(Label::new(text));
        self.insert(&mut label, &XY::new(position.0, position.1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset_server::TRANSPARENT_CHAR,
        drawable_objects::{drawable_object::DrawableObject, label::Label},
        utils::XY,
    };

    #[test]
    fn test_new_frame_assembler() {
        let frame_resolution = XY::new(5, 5);
        let background_char = '*';
        let frame_assembler = FrameAssembler::new(frame_resolution, background_char);

        assert_eq!(frame_assembler.frame.resolution.x, 5);
        assert_eq!(frame_assembler.frame.resolution.y, 5);
        for row in 0..frame_assembler.frame.resolution.y {
            for col in 0..frame_assembler.frame.resolution.x {
                assert_eq!(frame_assembler.frame.matrix[row][col], background_char);
            }
        }
    }

    #[test]
    fn test_new_default_frame_assembler() {
        let frame_resolution = XY::new(5, 5);
        let frame_assembler = FrameAssembler::new_default(frame_resolution);

        assert_eq!(frame_assembler.frame.resolution.x, 5);
        assert_eq!(frame_assembler.frame.resolution.y, 5);
        for row in 0..frame_assembler.frame.resolution.y {
            for col in 0..frame_assembler.frame.resolution.x {
                assert_eq!(frame_assembler.frame.matrix[row][col], TRANSPARENT_CHAR);
            }
        }
    }

    #[test]
    fn test_insert_drawable_object() {
        let mut frame_assembler = FrameAssembler::new(XY::new(5, 5), '*');
        
        let mut drawable = DrawableObject::Label(Label::new("Test"));
        let position = XY::new(1, 1);

        frame_assembler.insert(&mut drawable, &position);

        let frame = frame_assembler.get_frame();
        assert_eq!(frame.matrix[1][1], 'T');
        assert_eq!(frame.matrix[1][2], 'e');
        assert_eq!(frame.matrix[1][3], 's');
        assert_eq!(frame.matrix[1][4], 't');
    }

    #[test]
    fn test_label_insertion() {
        let mut frame_assembler = FrameAssembler::new(XY::new(5, 5), '*');
        
        frame_assembler.label("Hello", (0, 0));

        let frame = frame_assembler.get_frame();
        
        assert_eq!(frame.matrix[0][0], 'H');
        assert_eq!(frame.matrix[0][1], 'e');
        assert_eq!(frame.matrix[0][2], 'l');
        assert_eq!(frame.matrix[0][3], 'l');
        assert_eq!(frame.matrix[0][4], 'o');
    }

    #[test]
    fn test_insert_out_of_bounds() {
        let mut frame_assembler = FrameAssembler::new(XY::new(5, 5), '*');
        
        let mut drawable = DrawableObject::Label(Label::new("OutOfBounds"));

        let position = XY::new(4, 4);

        frame_assembler.insert(&mut drawable, &position);

        let frame = frame_assembler.get_frame();
        
        assert_eq!(frame.matrix[4][4], 'O');
        assert_eq!(frame.matrix[3][4], '*');
    }
}
