use std::{cmp::max, collections::HashMap, fs};
use crate::{
    bitmap::Bitmap,
    drawable_object::Sprite,
    utils::{self, XY},
};

pub const TRANSPARENT_CHAR: char = '$'; // works like png's transparency, do not confuse with space

pub struct AssetServer {
    assets: HashMap<String, Box<Sprite>>,
    asset_directory: String,
}
impl AssetServer {
    pub fn new(asset_directory: &str) -> Self {
        AssetServer {
            assets: HashMap::new(),
            asset_directory: asset_directory.to_owned(),
        }
    }

    pub fn load(&mut self, sprite_name: &str) -> Box<Sprite> {
        if let None = self.assets.get(sprite_name) {
            let new_sprite = SpriteFileReader::read(&(self.asset_directory.clone() + sprite_name));
            self.assets.insert(sprite_name.to_owned(), Box::new(new_sprite));
        }
        self.assets.get(sprite_name).unwrap().clone()
    }
}

struct SpriteFileReader;
impl SpriteFileReader {
    pub fn read(file_path: &str) -> Vec<Sprite> {
        let contents = fs::read_to_string(file_path);
        if let Err(_) = contents {
            utils::ErrorDisplayer::error(&format!("File not found at: {}", file_path));
        }
        // Sprite::new(&Self::parse_file_contents(&contents.unwrap()))
        todo!()
    }

    fn parse_file_contents(contents: &String) -> Vec<Bitmap<char>> {
        let lines: Vec<&str> = contents.lines().collect();
        let x_length = Self::find_line_length(&lines);
        let y_height = lines[0].parse::<usize>().unwrap();

        let groups = Self::split_into_groups(lines[1..].to_vec(), y_height);

        // Bitmap {
        //     resolution: XY::new(x_length, y_height),
        //     matrix,
        // }
        todo!()
    }

    fn find_line_length(lines: &Vec<&str>) -> usize {
        let mut max_length = 0;
        for line in lines.iter() {
            if line.len() > max_length {
                max_length = line.len();
            }
        }
        max_length
    }

    fn split_into_groups(lines: Vec<&str>, group_size: usize) -> Vec<Vec<&str>> {
        return lines
        .chunks(group_size)
        .map(|chunk| chunk.to_vec())
        .collect();
    }

    fn format_group(group: &Vec<&str>, size: usize) -> Vec<Vec<char>> {
        return group.iter()
            .map(|&line| {
                let mut chars: Vec<char> = line.chars().collect();
                chars.resize(size, '$');
                chars
            })
            .collect();
    }
}
