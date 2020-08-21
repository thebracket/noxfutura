use crate::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};

pub fn mapidx<N: Into<usize>>(x: N, y: N, z: N) -> usize {
    let xc = x.into();
    let yc = y.into();
    let zc = z.into();
    debug_assert!(xc <= REGION_WIDTH && yc <= REGION_HEIGHT && zc < REGION_DEPTH);
    (zc * REGION_HEIGHT as usize * REGION_WIDTH as usize) + (yc * REGION_WIDTH as usize) + xc
}

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
