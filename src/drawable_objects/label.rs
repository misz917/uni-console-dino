use super::drawable_object::Drawable;
use crate::{bitmap_utils::bitmap::Bitmap, utils::XY};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_creation() {
        let text = "Hello";
        let label = Label::new(text);

        assert_eq!(label.get_bitmap().resolution.x, 5);
        assert_eq!(label.get_bitmap().resolution.y, 1);

        let expected_matrix = vec![vec!['H', 'e', 'l', 'l', 'o']];
        assert_eq!(label.get_bitmap().matrix, expected_matrix);
    }

    #[test]
    fn test_label_empty_string() {
        let text = "";
        let label = Label::new(text);

        assert_eq!(label.get_bitmap().resolution.x, 0);
        assert_eq!(label.get_bitmap().resolution.y, 1);

        let expected_matrix: Vec<Vec<char>> = vec![vec![]];
        assert_eq!(label.get_bitmap().matrix, expected_matrix);
    }

    #[test]
    fn test_get_bitmap() {
        let text = "World";
        let label = Label::new(text);

        let bitmap = label.get_bitmap();

        assert_eq!(bitmap.resolution.x, 5);
        assert_eq!(bitmap.resolution.y, 1);
        assert_eq!(bitmap.matrix[0], vec!['W', 'o', 'r', 'l', 'd']);
    }
}
