use crate::utils::XY;
use std::collections::{HashMap, LinkedList};

struct HitBox {
    size: XY<usize>,
    position: XY<i32>,
}
impl HitBox {
    pub fn new(size: &XY<usize>, position: &XY<i32>) -> Self {
        HitBox {
            size: *size,
            position: *position,
        }
    }
}

pub struct CollisionDetector {
    objects: HashMap<String, HitBox>,
}
impl CollisionDetector {
    pub fn insert(&mut self, size: &XY<usize>, position: &XY<i32>, name: &str) {
        let new_object = HitBox::new(size, position);
        self.objects.insert(name.to_owned(), new_object);
    }

    pub fn does_collide(name: &str) -> bool {
        todo!()
    }
}
