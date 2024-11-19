use crate::{
    asset_server::AssetServer,
    bitmap_utils::{bitmap::Bitmap, frame_assembler::FrameAssembler},
    collision_detector::CollisionDetector,
    drawable_objects::drawable_object::{Drawable, DrawableObject},
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
        let mut ignore_name = false;
        if name == "*" {
            ignore_name = true;
        }
        self.replace_movement_function_logic(name, movement_function, ignore_name);
    }

    fn replace_movement_function_logic(
        &mut self,
        name: &str,
        movement_function: Option<MovementFunction>,
        ignore_name: bool,
    ) {
        for (_key, values) in self.objects.iter_mut() {
            for object in values {
                if ignore_name || object.name == name {
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

#[cfg(test)]
mod tests {
    use crate::drawable_objects::sprite::Sprite;

    use super::*;

    fn dummy_movement_fn(position: XY<i32>, time: f32, _optional_param: Option<f32>) -> XY<i32> {
        XY {
            x: position.x + (time * 5.0) as i32,
            y: position.y,
        }
    }

    #[test]
    fn test_movement_function_run_logic() {
        let movement_function = MovementFunction::new(dummy_movement_fn, None);
        let original_position = XY { x: 10, y: 20 };
        let time = 2.0;
        let new_position = movement_function.run_logic(original_position, time);

        assert_eq!(new_position.x, 20);
        assert_eq!(new_position.y, 20);
    }

    #[test]
    fn test_movement_function_with_optional_param() {
        let movement_function = MovementFunction::new(dummy_movement_fn, Some(2.0));
        let original_position = XY { x: 10, y: 20 };
        let time = 2.0;
        let new_position = movement_function.run_logic(original_position, time);

        assert_eq!(new_position.x, 20);
        assert_eq!(new_position.y, 20);
    }

    #[test]
    fn test_moving_object_creation() {
        let drawable_object = DrawableObject::Sprite(Sprite::new(&Bitmap::new(XY::new(5, 5), 'A')));
        let start_position = XY { x: 100, y: 150 };
        let movement_function = Some(MovementFunction::new(dummy_movement_fn, None));

        let moving_object = MovingObject::new(
            "Test Object",
            true,
            drawable_object,
            start_position,
            movement_function,
        );

        assert_eq!(moving_object.name, "Test Object");
        assert!(moving_object.can_collide);
        assert_eq!(moving_object.start_position.x, 100);
        assert_eq!(moving_object.start_position.y, 150);
    }

    #[test]
    fn test_moving_object_without_movement_function() {
        let drawable_object = DrawableObject::Sprite(Sprite::new(&Bitmap::new(XY::new(5, 5), 'A')));
        let start_position = XY { x: 100, y: 150 };

        let moving_object = MovingObject::new(
            "Test Object",
            true,
            drawable_object,
            start_position,
            None, // No movement function
        );

        assert_eq!(moving_object.name, "Test Object");
        assert!(moving_object.can_collide);
        assert_eq!(moving_object.start_position.x, 100);
        assert_eq!(moving_object.start_position.y, 150);
    }

    fn create_test_drawable() -> DrawableObject {
        DrawableObject::Sprite(Sprite::new(&Bitmap::new(XY::new(5, 5), 'A')))
    }

    #[test]
    fn test_insert_object() {
        let mut view = View::new("assets", '.');
        let name = "TestObject";
        let layer = 1;
        let start_position = XY { x: 10, y: 20 };
        let drawable_object = create_test_drawable();
        let movement_function = Some(MovementFunction::new(dummy_movement_fn, None));

        view.insert_object(
            name,
            layer,
            true,
            drawable_object,
            start_position,
            movement_function,
        );

        let objects = view.objects.get(&layer).unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].name, name);
        assert_eq!(objects[0].start_position, start_position);
    }

    #[test]
    fn test_replace_movement_function() {
        let mut view = View::new("assets", '.');
        let name = "TestObject";
        let layer = 1;
        let start_position = XY { x: 10, y: 20 };
        let drawable_object = create_test_drawable();
        let initial_movement_function = Some(MovementFunction::new(dummy_movement_fn, None));

        view.insert_object(
            name,
            layer,
            true,
            drawable_object,
            start_position,
            initial_movement_function,
        );

        let new_movement_function = Some(MovementFunction::new(
            |position, time, _| XY {
                x: position.x + (time * 3.0) as i32,
                y: position.y,
            },
            None,
        ));

        view.replace_movement_function(name, new_movement_function);

        let objects = view.objects.get(&layer).unwrap();
        let moving_object = &objects[0];

        assert_eq!(moving_object.mov_function.is_some(), true);
    }

    #[test]
    fn test_remove_object() {
        let mut view = View::new("assets", '.');
        let name = "TestObject";
        let layer = 1;
        let start_position = XY { x: 10, y: 20 };
        let drawable_object = create_test_drawable();
        let movement_function = Some(MovementFunction::new(dummy_movement_fn, None));

        view.insert_object(
            name,
            layer,
            true,
            drawable_object,
            start_position,
            movement_function,
        );
        view.remove_object(name);

        let objects = view.objects.get(&layer).unwrap();
        assert!(objects.is_empty());
    }

    #[test]
    fn test_check_for_collision() {
        let mut view = View::new("assets", '.');
        let name = "TestObject";
        let layer = 1;
        let start_position = XY { x: 10, y: 20 };
        let drawable_object = create_test_drawable();
        let movement_function = Some(MovementFunction::new(dummy_movement_fn, None));

        view.insert_object(
            name,
            layer,
            true,
            drawable_object,
            start_position,
            movement_function,
        );

        assert_eq!(view.check_for_collision(name), false);
    }
}
