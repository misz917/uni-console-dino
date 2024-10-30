use crate::utils::XY;

pub fn move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let new_x = original_position.x - (20.0 * time).round() as i32;
    let new_y = original_position.y;
    return XY::new(new_x, new_y);
}
