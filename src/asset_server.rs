use std::{collections::HashMap, fs};

use crate::{
    bitmap::Bitmap,
    utils::{self, Sprite, XY},
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
    pub fn read(file_path: &str) -> Bitmap<char> {
        let contents = fs::read_to_string(file_path);
        if let Err(_) = contents {
            utils::ErrorDisplayer::error(&format!("File not found at {}", file_path));
        }
        Self::parse_file_contents(&contents.unwrap())
    }

    fn parse_file_contents(contents: &String) -> Bitmap<char> {
        let parts: Vec<&str> = contents.split(';').collect();
        if parts.len() != 2 {
            utils::ErrorDisplayer::error("Sprite file format error");
        }
        let resolution = Self::parse_header(parts[0]);
        let map = Self::parse_tail(parts[1], &resolution);
        Bitmap { resolution, map }
    }

    fn parse_header(header: &str) -> XY<usize> {
        let numbers: Vec<&str> = header.split(':').collect();
        XY {
            x: numbers[0].parse::<usize>().unwrap(),
            y: numbers[1].parse::<usize>().unwrap(),
        }
    }

    // no idea how it works but it does
    fn parse_tail(tail: &str, resolution: &XY<usize>) -> Vec<Vec<char>> {
        let mut output_array: Vec<Vec<char>> = vec![vec![' '; resolution.y]; resolution.x];
        for (col, line) in tail.split_whitespace().enumerate() {
            for row in 0..resolution.x {
                if row < line.len() {
                    output_array[col][row] = line.chars().nth(row).unwrap();
                } else {
                    output_array[col][row] = ' ';
                }
            }
        }
        output_array
    }
}
