use std::collections::HashMap;

use crate::{
    asset_server::AssetServer,
    bitmap::Bitmap,
    drawable_object::DrawableObject,
    frame_assembler::{self, FrameAssembler},
    utils::XY,
    WINDOW_RESOLUTION,
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

pub struct MovingObject {
    pub object: DrawableObject,
    pub position: XY<i32>,
    pub mov_function: Option<MovementFunction>,
}

pub struct View {
    objects: HashMap<String, MovingObject>,
    asset_server: AssetServer,
}
impl View {
    pub fn new(asset_server: AssetServer) -> Self {
        View {
            objects: HashMap::new(),
            asset_server,
        }
    }

    pub fn insert_asset(
        &mut self,
        asset_name: &str,
        access_code: &str,
        position: XY<i32>,
        mov_function: MovementFunction,
    ) {
        let new_asset = self.asset_server.load(asset_name);
        self.objects.insert(
            access_code.to_owned(),
            MovingObject {
                object: *new_asset,
                position,
                mov_function: Some(mov_function),
            },
        );
    }

    pub fn compile(&mut self) -> Box<Bitmap<char>> {
        let mut frame_assembler = FrameAssembler::new(WINDOW_RESOLUTION);
        todo!()
    }
}
