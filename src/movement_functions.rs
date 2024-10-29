use crate::utils::XY;

pub fn zero(_original_position: XY<i32>, _time: f32) -> XY<i32> {
    return XY::new(0, 0);
}

pub fn move_right(original_position: XY<i32>, time: f32) -> XY<i32> {
    let new_x = original_position.x + time.round() as i32;
    let new_y = original_position.y;
    return XY::new(new_x, new_y);
}