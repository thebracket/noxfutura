use crate::planet::{REGION_HEIGHT, REGION_WIDTH, WORLD_HEIGHT, WORLD_WIDTH};

pub(crate) fn lat_to_y(lat: f32) -> i32 {
    let mut y = (((lat + 90.0) / 180.0) * WORLD_HEIGHT as f32) as i32;
    if y < 0 {
        y = WORLD_HEIGHT as i32 + y
    }
    if y > WORLD_HEIGHT as i32 - 1 {
        y -= WORLD_HEIGHT as i32
    }
    y
}

pub(crate) fn lon_to_x(lon: f32) -> i32 {
    let mut x = (((lon + 180.0) / 360.0) * WORLD_WIDTH as f32) as i32;
    if x < 0 {
        x = WORLD_WIDTH as i32 + x
    }
    if x > WORLD_WIDTH as i32 - 1 {
        x -= WORLD_WIDTH as i32
    }
    x
}

pub(crate) fn noise_lon(world_x: i32, region_x: i32) -> f32 {
    let x_extent = world_x as f32 / WORLD_WIDTH as f32;
    let sub_x = region_x as f32 / REGION_WIDTH as f32;
    let longitude = (x_extent * 360.0) + sub_x - 180.0;
    longitude

    //let big_x = ((world_x * WORLD_WIDTH as i32) + region_x) as f32;
    //(big_x / WORLD_WIDTH as f32 * REGION_WIDTH as f32) * NOISE_SIZE
}

pub(crate) fn noise_lat(world_y: i32, region_y: i32) -> f32 {
    let y_extent = world_y as f32 / WORLD_HEIGHT as f32;
    let sub_y = region_y as f32 / REGION_HEIGHT as f32;
    let latitude = (y_extent * 180.0) + sub_y - 90.0;
    latitude

    //let big_y = ((world_y * WORLD_HEIGHT as i32) + region_y) as f32;
    //(big_y / WORLD_HEIGHT as f32 * REGION_HEIGHT as f32) * NOISE_SIZE
}

pub(crate) fn noise_to_planet_height(n: f32) -> u8 {
    ((n + 1.0) * 150.0) as u8
}
