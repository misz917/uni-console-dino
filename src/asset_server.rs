use std::{collections::HashMap, fs};

use crate::{
    bitmap::Bitmap,
    utils::{self, Sprite, XY},
};

pub const TRANSPARENT_CHAR: char = '$'; // do not confuse with space

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
            utils::ErrorDisplayer::error(&format!("File not found at: {}", file_path));
        }
        Self::parse_file_contents(&contents.unwrap())
    }

    fn parse_file_contents(contents: &String) -> Bitmap<char> {
        let lines: Vec<&str> = contents.lines().collect();
        let resolution = XY {
            x: lines[0].parse::<usize>().unwrap(),
            y: lines.len() - 1,
        };
        let char_matrix: Vec<Vec<char>> = lines
            .iter()
            .skip(1)
            .map(|&line| {
                let mut chars: Vec<char> = line.chars().collect();
                chars.resize(16, '$');
                chars
            })
            .collect();

        Bitmap {
            resolution,
            matrix: char_matrix,
        }
    }
}
