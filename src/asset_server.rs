use std::{collections::HashMap, fs};

use crate::{
    bitmap::Bitmap,
    utils::{self, Sprite, XY},
};

pub const TRANSPARENT_CHAR: char = '$'; // works like png's transparency, do not confuse with space

pub struct AssetServer {
    assets: HashMap<String, Sprite>,
    asset_directory: String,
}
impl AssetServer {
    pub fn new(asset_directory: &str) -> Self {
        AssetServer {
            assets: HashMap::new(),
            asset_directory: asset_directory.to_owned(),
        }
    }

    pub fn load(&mut self, sprite_name: &str) -> &Sprite {
        if let None = self.assets.get(sprite_name) {
            let new_sprite = SpriteFileReader::read(&(self.asset_directory.clone() + sprite_name));
            self.assets.insert(sprite_name.to_owned(), new_sprite);
        }
        self.assets.get(sprite_name).unwrap()
    }
}

struct SpriteFileReader;
impl SpriteFileReader {
    pub fn read(file_path: &str) -> Sprite {
        let contents = fs::read_to_string(file_path);
        if let Err(_) = contents {
            utils::ErrorDisplayer::error(&format!("File not found at: {}", file_path));
        }
        Sprite::from_bitmap(&Self::parse_file_contents(&contents.unwrap()))
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
