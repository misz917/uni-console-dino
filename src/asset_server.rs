use std::{cmp::max, collections::HashMap, fs};
use crate::{
    bitmap::Bitmap,
    drawable_object::{Animation, DrawableObject, Sprite},
    utils::{self, XY},
};

pub const TRANSPARENT_CHAR: char = '$'; // works like png's transparency, do not confuse with space

pub struct AssetServer {
    assets: HashMap<String, Box<DrawableObject>>,
    asset_directory: String,
}
impl AssetServer {
    pub fn new(asset_directory: &str) -> Self {
        AssetServer {
            assets: HashMap::new(),
            asset_directory: asset_directory.to_owned(),
        }
    }

    pub fn load(&mut self, object_name: &str) -> Box<DrawableObject> {
        if let None = self.assets.get(object_name) {
            let new_object = SpriteFileReader::read(&(self.asset_directory.clone() + object_name));
            self.assets.insert(object_name.to_owned(), new_object);
        }
        self.assets.get(object_name).unwrap().clone()
    }
}

struct SpriteFileReader;
impl SpriteFileReader {
    pub fn read(file_path: &str) -> Box<DrawableObject> {
        let contents = fs::read_to_string(file_path);
        if let Err(_) = contents {
            utils::ErrorDisplayer::error(&format!("File not found at: {}", file_path));
        }
        let d_object = Self::parse_file_contents(&contents.unwrap());
        return d_object;
    }

    fn parse_file_contents(contents: &String) -> Box<DrawableObject> {
        let lines: Vec<&str> = contents.lines().collect();
        let x_length = Self::find_line_length(&lines);
        let y_height = lines[0].parse::<usize>().unwrap();

        let groups = Self::split_into_groups(lines[1..].to_vec(), y_height);
        let formatted_groups: Vec<Vec<Vec<char>>> = groups.iter().map(|group| {
            let formatted = Self::format_group(&group, x_length);
            formatted
        }).collect();

        if formatted_groups.len() > 1 {
            let frames: Vec<Bitmap<char>> = formatted_groups.iter().map(|group| {
                let bitmap = Bitmap {
                    resolution: XY::new(x_length, y_height),
                    matrix: group.clone(),
                };
                bitmap
            }).collect();
            let d_object = DrawableObject::Animation(Animation::new(&frames));
            return Box::new(d_object);
        } else {
            let bitmap = Bitmap {
                resolution: XY::new(x_length, y_height),
                matrix: formatted_groups[0].clone(),
            };
            let d_object = DrawableObject::Sprite(Sprite::new(&bitmap));
            return Box::new(d_object);
        }
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
