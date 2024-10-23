use std::{collections::HashMap, fs, process::exit};

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
            utils::ErrorDisplayer::error("File not found");
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
        let numbers: Vec<&str> = header.split('x').collect();
        XY {
            x: numbers[0].parse::<usize>().unwrap(),
            y: numbers[0].parse::<usize>().unwrap(),
        }
    }

    fn parse_tail(tail: &str, resolution: &XY<usize>) -> Vec<Vec<char>> {
        let mut output_array: Vec<Vec<char>> = vec![vec![' '; resolution.y]; resolution.x];
        for (y, line) in tail.split_whitespace().enumerate() {
            for x in 0..resolution.x {
                if x < line.len() {
                    output_array[x][y] = line.chars().nth(x).unwrap();
                } else {
                    output_array[x][y] = ' ';
                }
            }
        }
        output_array
    }
}
