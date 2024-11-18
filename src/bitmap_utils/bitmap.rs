use crate::utils::XY;

#[derive(Clone, Debug, PartialEq)]
pub struct Bitmap<T> {
    pub resolution: XY<usize>,
    pub matrix: Vec<Vec<T>>,
}
impl<T: Clone> Bitmap<T> {
    pub fn new(resolution: XY<usize>, default_contents: T) -> Self {
        Bitmap {
            resolution,
            matrix: vec![vec![default_contents.clone(); resolution.x]; resolution.y],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_creation() {
        let resolution = XY { x: 3, y: 3 };
        let bitmap: Bitmap<i32> = Bitmap::new(resolution, 0);

        assert_eq!(bitmap.resolution.x, 3);
        assert_eq!(bitmap.resolution.y, 3);

        assert_eq!(bitmap.matrix.len(), 3);
        assert_eq!(bitmap.matrix[0].len(), 3);
        for row in &bitmap.matrix {
            for &cell in row {
                assert_eq!(cell, 0);
            }
        }
    }

    #[test]
    fn test_bitmap_with_different_default_value() {
        let resolution = XY { x: 2, y: 2 };
        let bitmap: Bitmap<String> = Bitmap::new(resolution, "X".to_string());

        assert_eq!(bitmap.matrix.len(), 2);
        assert_eq!(bitmap.matrix[0].len(), 2);
        for row in &bitmap.matrix {
            for cell in row {
                assert_eq!(cell, "X");
            }
        }
    }
}
