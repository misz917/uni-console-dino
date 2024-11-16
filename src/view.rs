use crate::{
    asset_server::AssetServer,
    bitmap::Bitmap,
    collision_detector::CollisionDetector,
    drawable_object::{Drawable, DrawableObject},
    frame_assembler::FrameAssembler,
    utils::XY,
    WINDOW_RESOLUTION,
};
use std::{collections::BTreeMap, time::SystemTime};

#[derive(Clone)]
pub struct MovementFunction {
    function: fn(XY<i32>, f32, Option<f32>) -> XY<i32>,
    optional_param: Option<f32>,
}
impl MovementFunction {
    pub fn new(
        function: fn(XY<i32>, f32, Option<f32>) -> XY<i32>,
        optional_param: Option<f32>,
    ) -> Self {
        MovementFunction {
            function,
            optional_param,
        }
    }

    pub fn run_logic(&self, original_position: XY<i32>, time: f32) -> XY<i32> {
        (self.function)(original_position, time, self.optional_param)
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
    objects: BTreeMap<i16, Vec<MovingObject>>,
    asset_server: AssetServer,
    default_background: char,
    collision_detector: CollisionDetector,
}
impl View {
    pub fn new(asset_directory: &str, default_background: char) -> Self {
        View {
            objects: BTreeMap::new(),
            asset_server: AssetServer::new(asset_directory),
            default_background,
            collision_detector: CollisionDetector::new(),
        }
    }

    pub fn insert_asset(
        &mut self,
        name: &str,
        layer: i16,
        can_collide: bool,
        asset_path: &str,
        start_position: XY<i32>,
        movement_function: Option<MovementFunction>,
    ) {
        let mut drawable_object = self.asset_server.load(asset_path);
        match *drawable_object {
            DrawableObject::Animation(ref mut animation) => animation.reset_time(),
            _ => (),
        }
        let moving_object = MovingObject::new(
            name,
            can_collide,
            *drawable_object,
            start_position,
            movement_function,
        );
        self.objects
            .entry(layer)
            .or_insert_with(Vec::new)
            .push(moving_object);
    }

    pub fn insert_object(
        &mut self,
        name: &str,
        layer: i16,
        can_collide: bool,
        drawable_object: DrawableObject,
        start_position: XY<i32>,
        movement_function: Option<MovementFunction>,
    ) {
        let moving_object = MovingObject::new(
            name,
            can_collide,
            drawable_object,
            start_position,
            movement_function,
        );
        self.objects
            .entry(layer)
            .or_insert_with(Vec::new)
            .push(moving_object);
    }

    pub fn replace_movement_function(
        &mut self,
        name: &str,
        movement_function: Option<MovementFunction>,
    ) {
        for (_key, values) in self.objects.iter_mut() {
            for object in values {
                if object.name == name {
                    object.mov_function = movement_function.clone();
                    object.clock = SystemTime::now();
                }
            }
        }
    }

    pub fn remove_object(&mut self, name: &str) {
        if name == "*" {
            self.objects.clear();
        } else {
            self.remove_object_logic(name);
        }
    }

    fn remove_object_logic(&mut self, name: &str) {
        for (_key, values) in self.objects.iter_mut() {
            let mut i = values.len();
            if i == 0 {
                continue;
            }
            i -= 1;
            loop {
                if values[i].name == name {
                    values.swap_remove(i);
                }
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
    }

    pub fn compile(&mut self) -> Box<Bitmap<char>> {
        let mut frame_assembler = FrameAssembler::new(WINDOW_RESOLUTION, self.default_background);
        self.collision_detector.empty();

        for (_key, values) in self.objects.iter_mut() {
            for object in values {
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
                } else {
                    self.collision_detector.special_insert(
                        &object.drawable_object.get_bitmap().resolution,
                        &modified_position,
                        &object.name,
                    );
                }
                frame_assembler.insert(&mut object.drawable_object, &modified_position);
            }
        }

        return frame_assembler.get_frame();
    }

    pub fn check_for_collision(&self, name: &str) -> bool {
        self.collision_detector.check_for_collisions(name)
    }

    pub fn check_for_collision_between(&self, name_a: &str, name_b: &str) -> bool {
        self.collision_detector
            .check_for_collision_between(name_a, name_b)
    }
}
