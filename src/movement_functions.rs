use crate::{utils::XY, WINDOW_RESOLUTION};

pub fn zero(_time: f32) -> XY<i32> {
    return XY::new(0, 0);
}

pub fn move_right(_time: f32) -> XY<i32> {
    let new_x = _time.round() as i32;
    let new_y = _time.round() as i32;
    return XY::new(new_x, new_y);
}