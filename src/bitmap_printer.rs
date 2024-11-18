use crate::{
    asset_server::TRANSPARENT_CHAR,
    bitmap::Bitmap,
    utils::{ESC, XY},
};

pub trait Printer {
    fn print(&self, bitmap: &Bitmap<char>, border_width: &XY<usize>);
}

pub struct BitmapPrinter;
impl Printer for BitmapPrinter {
    fn print(&self, bitmap: &Bitmap<char>, border_width: &XY<usize>) {
        for (i, row) in bitmap.matrix.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == TRANSPARENT_CHAR {
                    continue;
                }
                print!(
                    "{}[{};{}H{}",
                    ESC,
                    i + 1 + border_width.y,
                    j + 1 + border_width.x,
                    item
                );
            }
        }
    }
}
