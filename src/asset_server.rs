
use std::collections::HashMap;

use crate::utils::Sprite;

pub struct AssetServer {
    assets: HashMap<String, Sprite>,
}
impl AssetServer {
    pub fn new() -> Self {
        AssetServer {
            assets: HashMap::new()
        }
    }

    pub fn load(&self, sprite_name: &String) -> &Sprite {
        match self.assets.get(sprite_name) {
            Some(sprite) => return sprite,
            None => todo!(),
        }
    }
}