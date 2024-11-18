use super::drawable_object::Drawable;
use crate::{bitmap_utils::bitmap::Bitmap, utils::XY};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_creation() {
        let dimensions = XY { x: 4, y: 3 };
        let filling = '*';
        let rectangle = Rectangle::new(dimensions, filling);

        assert_eq!(rectangle.get_bitmap().resolution.x, 4);
        assert_eq!(rectangle.get_bitmap().resolution.y, 3);

        assert_eq!(rectangle.get_bitmap().matrix.len(), 3);
        assert_eq!(rectangle.get_bitmap().matrix[0].len(), 4);

        for row in &rectangle.get_bitmap().matrix {
            for &cell in row {
                assert_eq!(cell, '*');
            }
        }
    }

    #[test]
    fn test_rectangle_empty() {
        let dimensions = XY { x: 0, y: 0 };
        let filling = '#';
        let rectangle = Rectangle::new(dimensions, filling);

        assert_eq!(rectangle.get_bitmap().resolution.x, 0);
        assert_eq!(rectangle.get_bitmap().resolution.y, 0);

        assert!(rectangle.get_bitmap().matrix.is_empty());
    }

    #[test]
    fn test_get_bitmap() {
        let dimensions = XY { x: 5, y: 2 };
        let filling = '@';
        let rectangle = Rectangle::new(dimensions, filling);

        let bitmap = rectangle.get_bitmap();

        assert_eq!(bitmap.resolution.x, 5);
        assert_eq!(bitmap.resolution.y, 2);

        for row in &bitmap.matrix {
            for &cell in row {
                assert_eq!(cell, '@');
            }
        }
    }
}
