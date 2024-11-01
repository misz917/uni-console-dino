use crate::utils::XY;

pub fn move_left(original_position: XY<i32>, time: f32) -> XY<i32> {
    let new_x = original_position.x - (20.0 * time).round() as i32;
    let new_y = original_position.y;
    return XY::new(new_x, new_y);
}

pub fn jump(original_position: XY<i32>, time: f32) -> XY<i32> {
    // let func = |x: f32| -(x - 3.0).powf(2.0) + 9.0;
    let func = |x: f32| -((x - 6.34).powf(2.0)) / 4.0 + 10.0;

    let new_x = original_position.x;
    let new_y = original_position.y - func(time * 9.0).round() as i32;
    return XY::new(new_x, new_y);
}
