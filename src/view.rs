use std::time::SystemTime;

use crate::{
    asset_server::AssetServer,
    bitmap::Bitmap,
    drawable_object::DrawableObject,
    frame_assembler::FrameAssembler,
    utils::XY,
    WINDOW_RESOLUTION,
};

pub struct MovementFunction(fn(XY<i32>, f32) -> XY<i32>);
impl MovementFunction {
    pub fn new(function: fn(XY<i32>, f32) -> XY<i32>) -> Self {
        MovementFunction(function)
    }

    pub fn calculate_position(&self, original_position: XY<i32>, time: f32) -> XY<i32> {
        (self.0)(original_position, time)
    }
}

pub struct MovingObject {
    pub drawable_object: DrawableObject,
    pub start_position: XY<i32>,
    pub mov_function: Option<MovementFunction>,
    pub clock: SystemTime,
}
impl MovingObject {
    pub fn new(
        drawable_object: DrawableObject,
        start_position: XY<i32>,
        mov_function: Option<MovementFunction>
    ) -> Self {
        MovingObject {
            drawable_object,
            start_position,
            mov_function,
            clock: SystemTime::now(),
        }
    }
}

pub struct View {
    objects: Vec<MovingObject>,
    asset_server: AssetServer,
}
impl View {
    pub fn new(asset_directory: &str) -> Self {
        View {
            objects: Vec::new(),
            asset_server: AssetServer::new(asset_directory)
        }
    }

    pub fn insert_asset(
        &mut self,
        asset_name: &str,
        start_position: XY<i32>,
        mov_function: Option<MovementFunction>
    ) {
        let drawable_object = self.asset_server.load(asset_name);
        let moving_object = MovingObject::new(*drawable_object, start_position, mov_function);
        self.objects.push(moving_object);
    }

    pub fn insert_object() {
        todo!()
    }

    pub fn compile(&mut self) -> Box<Bitmap<char>> {
        let mut frame_assembler = FrameAssembler::new(WINDOW_RESOLUTION);
        for object in self.objects.iter_mut() {
            let mut modified_position = object.start_position;
            if let Some(movement_function) = &object.mov_function {
                modified_position = movement_function.calculate_position(object.start_position, object.clock.elapsed().unwrap().as_secs_f32());
            }
            frame_assembler.insert(&object.drawable_object, &modified_position);
        }
        return frame_assembler.get_frame();
    }
}
