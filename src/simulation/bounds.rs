#![allow(dead_code)]

// World Size
pub const WORLD_HEIGHT: usize = 90;
pub const WORLD_WIDTH: usize = 180;
pub const WORLD_TILES_COUNT: usize = WORLD_HEIGHT * WORLD_WIDTH;

// Region Size
pub const REGION_WIDTH: usize = 256;
pub const REGION_HEIGHT: usize = 256;
pub const REGION_DEPTH: usize = 256;
pub const REGION_TILES_COUNT: usize = (REGION_WIDTH * REGION_HEIGHT * REGION_DEPTH) as usize;

/// Indexes a planet-level block
pub fn planet_idx<N: Into<usize>>(x: N, y: N) -> usize {
    let xc = x.into();
    let yc = y.into();
    debug_assert!(xc < WORLD_WIDTH && yc < WORLD_HEIGHT);
    (WORLD_WIDTH * yc) + xc
}

// Indexes a planet-level block id back to x/y
pub fn idx_planet(idx: usize) -> (usize, usize) {
    (idx % WORLD_WIDTH, idx / WORLD_WIDTH)
}

/// Indexes a map tile within an active map
pub fn mapidx<N: Into<usize>>(x: N, y: N, z: N) -> usize {
    let xc = x.into();
    let yc = y.into();
    let zc = z.into();
    debug_assert!(xc <= REGION_WIDTH && yc <= REGION_HEIGHT && zc < REGION_DEPTH);
    (zc * REGION_HEIGHT as usize * REGION_WIDTH as usize) + (yc * REGION_WIDTH as usize) + xc
}

/// Reverse-Indexes a map tile within an active map
pub fn idxmap(mut idx: usize) -> (usize, usize, usize) {
    debug_assert!(idx < REGION_DEPTH * REGION_WIDTH * REGION_HEIGHT);
    const LAYER_SIZE: usize = REGION_WIDTH as usize * REGION_HEIGHT as usize;
    let z = idx / LAYER_SIZE;
    idx -= z * LAYER_SIZE;

    let y = idx / REGION_WIDTH as usize;
    idx -= y * REGION_WIDTH as usize;

    let x = idx;
    debug_assert!(x <= REGION_WIDTH && y <= REGION_HEIGHT && z < REGION_DEPTH);
    (x, y, z)
}

pub fn lat_to_y(lat: f32) -> usize {
    let mut y = (((lat + 90.0) / 180.0) * WORLD_HEIGHT as f32) as usize;
    if y > WORLD_HEIGHT - 1 {
        y -= WORLD_HEIGHT
    }
    y
}

pub fn lon_to_x(lon: f32) -> usize {
    (((lon + 180.0) / 360.0) * WORLD_WIDTH as f32) as usize
}

pub fn noise_lon(world_x: usize, region_x: usize) -> f32 {
    let x_extent = world_x as f32 / WORLD_WIDTH as f32;
    let sub_x = region_x as f32 / REGION_WIDTH as f32;
    let longitude = (x_extent * 360.0) + (sub_x - 0.5) - 180.0;
    longitude
}

pub fn noise_lat(world_y: usize, region_y: usize) -> f32 {
    let y_extent = world_y as f32 / WORLD_HEIGHT as f32;
    let sub_y = region_y as f32 / REGION_HEIGHT as f32;
    let latitude = (y_extent * 180.0) + (sub_y - 0.5) - 90.0;
    latitude
}
