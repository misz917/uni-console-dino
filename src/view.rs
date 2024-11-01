use crate::{
    asset_server::AssetServer, bitmap::Bitmap, collision_detector::{self, CollisionDetector}, drawable_object::{Drawable, DrawableObject}, frame_assembler::FrameAssembler, utils::XY, WINDOW_RESOLUTION
};
use std::time::SystemTime;

pub struct MovementFunction(fn(XY<i32>, f32) -> XY<i32>);
impl MovementFunction {
    pub fn new(function: fn(XY<i32>, f32) -> XY<i32>) -> Self {
        MovementFunction(function)
    }

    pub fn run_logic(&self, original_position: XY<i32>, time: f32) -> XY<i32> {
        (self.0)(original_position, time) // let the function delete the object, make it work first and only later make it elegant
    }
}

pub struct MovingObject {
    drawable_object: DrawableObject,
    start_position: XY<i32>,
    mov_function: Option<MovementFunction>,
    // hitbox_size: XY<usize>, // dimensions counted from upper-left corner
    clock: SystemTime,
}
impl MovingObject {
    pub fn new(
        drawable_object: DrawableObject,
        start_position: XY<i32>,
        mov_function: Option<MovementFunction>,
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
    objects: Vec<(String, MovingObject)>,
    asset_server: AssetServer,
    default_background: char,
    collision_detector: CollisionDetector,
}
impl View {
    pub fn new(asset_directory: &str, default_background: char) -> Self {
        View {
            objects: Vec::new(),
            asset_server: AssetServer::new(asset_directory),
            default_background,
            collision_detector: CollisionDetector::new()
        }
    }

    pub fn insert_asset(
        &mut self,
        asset_name: &str,
        asset_path: &str,
        start_position: XY<i32>,
        movement_function: Option<MovementFunction>,
    ) {
        let drawable_object = self.asset_server.load(asset_path);
        let moving_object = MovingObject::new(*drawable_object, start_position, movement_function);
        self.objects.push((asset_name.to_owned(), moving_object));
    }

    pub fn insert_object() {
        todo!() // implement later if needed
    }

    pub fn compile(&mut self) -> Box<Bitmap<char>> {
        let mut frame_assembler = FrameAssembler::new(WINDOW_RESOLUTION, self.default_background);
        self.collision_detector.empty();

        for object in self.objects.iter_mut() {
            let mut modified_position = object.1.start_position;
            if let Some(movement_function) = &object.1.mov_function {
                modified_position = movement_function.run_logic(
                    object.1.start_position,
                    object.1.clock.elapsed().unwrap().as_secs_f32(),
                );
            }
            self.collision_detector.insert(&object.1.drawable_object.get_bitmap().resolution, &modified_position, &object.0);
            frame_assembler.insert(&object.1.drawable_object, &modified_position);
        }

        return frame_assembler.get_frame();
    }
}
