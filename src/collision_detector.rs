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

    pub fn does_collide(&self, name: &str) -> Option<bool> {
        if let Some(compared_object) = self.objects.get(name) {
            for object in &self.objects {
                todo!()
            }
            todo!()
        } else {
            return None;
        }
    }

    fn check_collision(
        size_a: XY<usize>,
        position_a: XY<i32>,
        size_b: XY<usize>,
        position_b: XY<i32>,
    ) -> bool {
        let a_left = position_a.x;
        let a_right = position_a.x + size_a.x as i32;
        let a_top = position_a.y;
        let a_bottom = position_a.y + size_a.y as i32;

        let b_left = position_b.x;
        let b_right = position_b.x + size_b.x as i32;
        let b_top = position_b.y;
        let b_bottom = position_b.y + size_b.y as i32;

        return !(a_left >= b_right || a_right <= b_left || a_top >= b_bottom || a_bottom <= b_top);
    }
}
