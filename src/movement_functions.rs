use crate::{utils::XY, WINDOW_RESOLUTION};

pub fn zero(_time: i32) -> XY<i32> {
    return XY::new(0, 0);
}

pub fn move_left(time: i32) -> XY<i32> {
    let new_x = WINDOW_RESOLUTION.x as i32;
    let new_y = WINDOW_RESOLUTION.y as i32;
    return XY::new(new_x, new_y);
}