use std::collections::HashMap;

use crate::{
    asset_server::AssetServer, bitmap::Bitmap, drawable_object::DrawableObject, utils::XY,
};

pub struct MovementFunction(fn(i32) -> XY<usize>);
impl MovementFunction {
    pub fn new(function: fn(i32) -> XY<usize>) -> Self {
        MovementFunction(function)
    }

    pub fn calculate_position(&self, val: i32) -> XY<usize> {
        (self.0)(val)
    }
}

pub struct View {
    objects: HashMap<String, (DrawableObject, Option<MovementFunction>)>,
    asset_server: AssetServer,
}
impl View {
    pub fn new(asset_server: AssetServer) -> Self {
        View {
            objects: HashMap::new(),
            asset_server,
        }
    }

    pub fn insert_asset(&mut self, asset_name: &str, access_code: &str) {
        let new_asset = self.asset_server.load(asset_name);
        self.objects
            .insert(access_code.to_owned(), (*new_asset, None));
    }

    pub fn assemble() -> Box<Bitmap<char>> {
        todo!()
    }
}
