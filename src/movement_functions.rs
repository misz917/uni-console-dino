use crate::{utils::XY, SPEED};

pub fn move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let speed = SPEED.lock().unwrap();
    let new_x = original_position.x - (*speed * 30.0 * time).round() as i32;
    let new_y = original_position.y;
    return XY::new(new_x, new_y);
}

pub fn jump(original_position: XY<i32>, time: f32) -> XY<i32> {
    let func = |x: f32| -(x / 2.0 - 3.18).powf(2.0) + 10.0;
    let mut difference = func(time * 8.0).round() as i32;
    if difference < 0 {
        difference = 0;
    }

    let new_x = original_position.x;
    let new_y = original_position.y - difference;
    return XY::new(new_x, new_y);
}
