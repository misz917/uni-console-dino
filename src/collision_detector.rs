use crate::utils::XY;
use std::collections::HashMap;

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
    pub fn new() -> Self {
        CollisionDetector {
            objects: HashMap::new(),
        }
    }

    pub fn insert(&mut self, size: &XY<usize>, position: &XY<i32>, name: &str) {
        let new_object = HitBox::new(size, position);
        self.objects.insert(name.to_owned(), new_object);
    }

    pub fn does_collide(&self, name: &str) -> Option<bool> {
        if let Some(compared_object) = self.objects.get(name) {
            for (object_name, object_hitbox) in &self.objects {
                if object_name == name {
                    continue;
                } else if Self::check_collision(compared_object, object_hitbox) {
                    return Some(true);
                }
            }
            return Some(false);
        } else {
            return None;
        }
    }

    fn check_collision(object_a: &HitBox, object_b: &HitBox) -> bool {
        let a_left = object_a.position.x;
        let a_right = object_a.position.x + object_a.size.x as i32;
        let a_top = object_a.position.y;
        let a_bottom = object_a.position.y + object_a.size.y as i32;

        let b_left = object_b.position.x;
        let b_right = object_b.position.x + object_b.size.x as i32;
        let b_top = object_b.position.y;
        let b_bottom = object_b.position.y + object_b.size.y as i32;

        return !(a_left >= b_right || a_right <= b_left || a_top >= b_bottom || a_bottom <= b_top);
    }
}