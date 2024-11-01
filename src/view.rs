use crate::{
    asset_server::AssetServer,
    bitmap::Bitmap,
    collision_detector::CollisionDetector,
    drawable_object::{Drawable, DrawableObject},
    frame_assembler::FrameAssembler,
    utils::XY,
    WINDOW_RESOLUTION,
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
    name: String,
    can_collide: bool,
    drawable_object: DrawableObject,
    start_position: XY<i32>,
    mov_function: Option<MovementFunction>,
    clock: SystemTime,
}
impl MovingObject {
    pub fn new(
        name: &str,
        can_collide: bool,
        drawable_object: DrawableObject,
        start_position: XY<i32>,
        mov_function: Option<MovementFunction>,
    ) -> Self {
        MovingObject {
            name: name.to_owned(),
            can_collide,
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
    default_background: char,
    collision_detector: CollisionDetector,
}
impl View {
    pub fn new(asset_directory: &str, default_background: char) -> Self {
        View {
            objects: Vec::new(),
            asset_server: AssetServer::new(asset_directory),
            default_background,
            collision_detector: CollisionDetector::new(),
        }
    }

    pub fn insert_asset(
        &mut self,
        name: &str,
        can_collide: bool,
        asset_path: &str,
        start_position: XY<i32>,
        movement_function: Option<MovementFunction>,
    ) {
        let drawable_object = self.asset_server.load(asset_path);
        let moving_object = MovingObject::new(
            name,
            can_collide,
            *drawable_object,
            start_position,
            movement_function,
        );
        self.objects.push(moving_object);
    }

    // untested, unused
    pub fn remove_object(&mut self, name: &str) {
        let mut found_index: Option<usize> = None;
        for (index, object) in self.objects.iter().enumerate() {
            if object.name == name {
                found_index = Some(index);
                break;
            }
        }
        if let Some(index) = found_index {
            self.objects.swap_remove(index);
        }
    }

    pub fn insert_object() {
        todo!() // implement later if needed
    }

    pub fn compile(&mut self) -> Box<Bitmap<char>> {
        let mut frame_assembler = FrameAssembler::new(WINDOW_RESOLUTION, self.default_background);
        self.collision_detector.empty();

        for object in self.objects.iter_mut() {
            let mut modified_position = object.start_position;
            if let Some(movement_function) = &object.mov_function {
                modified_position = movement_function.run_logic(
                    object.start_position,
                    object.clock.elapsed().unwrap().as_secs_f32(),
                );
            }
            if object.can_collide {
                self.collision_detector.insert(
                    &object.drawable_object.get_bitmap().resolution,
                    &modified_position,
                    &object.name,
                );
            }
            frame_assembler.insert(&object.drawable_object, &modified_position);
        }

        return frame_assembler.get_frame();
    }

    pub fn check_collision(&self, name: &str) -> bool {
        if let Some(b) = self.collision_detector.does_collide(name) {
            return b;
        }
        return false;
    }
}
