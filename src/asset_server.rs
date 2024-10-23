use std::{collections::HashMap, fs, process::exit};

use crate::{
    bitmap::Bitmap,
    utils::{Sprite, XY},
    WINDOW_RESOLUTION,
};

pub struct AssetServer {
    assets: HashMap<String, Sprite>,
}
impl AssetServer {
    pub fn new() -> Self {
        AssetServer {
            assets: HashMap::new(),
        }
    }

    pub fn load(&self, sprite_name: &String) -> &Sprite {
        match self.assets.get(sprite_name) {
            Some(sprite) => return sprite,
            None => todo!(),
        }
    }
}

pub struct SpriteFileReader;
impl SpriteFileReader {
    pub fn read(file_path: &str) -> &Bitmap<char> {
        let contents = fs::read_to_string(file_path).expect("Could not read file");

        // let resolution = Self::parse_header(parts[0]);
        // let header: String = split_contents.split('x').collect();
        // let dimensions: XY<usize> = XY::new(header, y)
        // todo!()
        // Self::parse_file_contents(&contents)
        let mut output_bitmap: Bitmap<char>;
        // Self::parse_file_contents(&contents, &mut output_bitmap);
        todo!()
    }

    fn parse_file_contents(contents: &String, output_bitmap: &mut Bitmap<char>) {
        let parts: Vec<&str> = contents.split(';').collect();
        if parts.len() != 2 {
            exit(2);
        }
        let resolution = Self::parse_header(parts[0]);
        Self::parse_tail(parts[1], &resolution, output_bitmap);
    }

    fn parse_header(header: &str) -> XY<usize> {
        let numbers: Vec<&str> = header.split('x').collect();
        XY {
            x: numbers[0].parse::<usize>().unwrap(),
            y: numbers[0].parse::<usize>().unwrap(),
        }
    }

    // returns an incomplete bitmap (without resolution)
    fn parse_tail(tail: &str, resolution: &XY<usize>, output_bitmap: &mut Bitmap<char>) {
        for i in 0..tail.len() {}
        todo!()
    }
}
