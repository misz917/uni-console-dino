use std::{collections::HashMap, time::SystemTime};

use crate::{
    asset_server::AssetServer,
    bitmap::Bitmap,
    drawable_object::{self, DrawableObject},
    frame_assembler::FrameAssembler,
    utils::XY,
    WINDOW_RESOLUTION,
};

pub struct MovementFunction(fn(i32) -> XY<i32>);
impl MovementFunction {
    pub fn new(function: fn(i32) -> XY<i32>) -> Self {
        MovementFunction(function)
    }

    pub fn calculate_position(&self, time: i32) -> XY<i32> {
        (self.0)(time)
    }
}

pub struct MovingObject {
    pub drawable_object: DrawableObject,
    pub start_position: XY<i32>,
    pub mov_function: Option<MovementFunction>,
    pub clock: SystemTime,
}
impl MovingObject {
    pub fn new(drawable_object: DrawableObject, start_position: XY<i32>, mov_function: MovementFunction) -> Self {
        MovingObject {
            drawable_object,
            start_position,
            mov_function: Some(mov_function),
            clock: SystemTime::now(),
        }
    }
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

    pub fn insert_object(
        &mut self,
        asset_name: &str,
        access_code: &str,
        start_position: XY<i32>,
        mov_function: MovementFunction,
    ) {
        let new_asset = self.asset_server.load(asset_name);
        self.objects.insert(
            access_code.to_owned(),
            MovingObject::new(*new_asset, start_position, mov_function),
        );
    }

    pub fn compile(&mut self) -> Box<Bitmap<char>> {
        let mut frame_assembler = FrameAssembler::new(WINDOW_RESOLUTION);
        todo!()
    }
}
