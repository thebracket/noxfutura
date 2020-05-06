const NOISE_SIZE : f32 = 384.0;
use crate::planet::{WORLD_HEIGHT, WORLD_WIDTH, REGION_HEIGHT, REGION_WIDTH};

pub(crate) fn noise_x(world_x: i32, region_x: i32) -> f32 {
    let big_x = ((world_x * WORLD_WIDTH as i32) + region_x) as f32;
    (big_x / WORLD_WIDTH as f32 * REGION_WIDTH as f32) * NOISE_SIZE
}

pub(crate) fn noise_y(world_y : i32, region_y : i32) -> f32 {
    let big_y = ((world_y * WORLD_HEIGHT as i32) + region_y) as f32;
    (big_y / WORLD_HEIGHT as f32 * REGION_HEIGHT as f32) * NOISE_SIZE
}

pub(crate) fn noise_to_planet_height(n: f32) -> u8 {
    ((n + 1.0) * 150.0) as u8
}