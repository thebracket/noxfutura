// World Size
pub const WORLD_HEIGHT: usize = 180;
pub const WORLD_WIDTH: usize = 360;
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