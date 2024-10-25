use crate::{
    asset_server::TRANSPARENT_CHAR,
    utils::{ESC, XY},
};

#[derive(Clone)]
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

pub trait Printer {
    fn print(&self, bitmap: &Bitmap<char>, border_width: &XY<usize>);
}

pub struct BitmapPrinter;
impl Printer for BitmapPrinter {
    fn print (&self, bitmap: &Bitmap<char>, border_width: &XY<usize>) {
        for (i, row) in bitmap.matrix.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == TRANSPARENT_CHAR {
                    continue;
                }
                print!("{}[{};{}H{}", ESC, i + 1 + border_width.y, j + 1 + border_width.x, item);
            }
        }
    }
}
