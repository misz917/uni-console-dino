use crate::utils::XY;

#[derive(Debug)]
struct HitBox {
    size: XY<usize>,
    position: XY<i32>,,
    name: String
}
impl HitBox {
    pub fn new(size: &XY<usize>, position: &XY<i32>, name: &str) -> Self {
        HitBox {
            size: *size,
            position: *position,
            name: name.to_owned(),
        }
    }
}

pub struct CollisionDetector {
    objects: Vec<HitBox>,
    special_objects: Vec<HitBox>,
}
impl CollisionDetector {
    pub fn new() -> Self {
        CollisionDetector {
            objects: Vec::new(),
            special_objects: Vec::new(),
        }
    }

    pub fn empty(&mut self) {
        self.objects.clear();
    }

    pub fn insert(&mut self, size: &XY<usize>, position: &XY<i32>, name: &str) {
        let new_object = HitBox::new(size, position, name);
        self.objects.push(new_object);
    }

    pub fn special_insert(&mut self, size: &XY<usize>, position: &XY<i32>, name: &str) {
        let new_object = HitBox::new(size, position, name);
        self.special_objects.push(new_object);
    }

    pub fn check_for_collisions(&self, name: &str) -> Option<bool> {
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

    pub fn check_for_collision_between(&self, name_a: &str, name_b: &str) -> bool {
        // let mut object_a: Option<&HitBox> = None;
        // let mut object_b: Option<&HitBox> = None;
        // if let Some(object) = self.objects.get(name_a) {
        //     object_a = Some(object);
        // } else if let Some(object) = self.special_objects.get(name_a) {
        //     object_a = Some(object);
        // }
        // if let Some(object) = self.objects.get(name_b) {
        //     object_b = Some(object);
        // } else if let Some(object) = self.special_objects.get(name_b) {
        //     object_b = Some(object);
        // }

        // if object_a.is_none() || object_b.is_none() {
        //     return false;
        // } else {
        //     return Self::check_collision(object_a.unwrap(), object_b.unwrap());
        // }
        
        // let objects_a: Vec<
    }
}
