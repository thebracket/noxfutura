use bracket_geometry::prelude::Radians;

pub fn sphere_vertex<A: Into<Radians>>(altitude: f32, lat: A, lon: A) -> (f32, f32, f32) {
    let rlat = lat.into();
    let rlon = lon.into();
    let sinlat = f32::sin(rlat.0);
    let coslat = f32::cos(rlat.0);
    let sinlon = f32::sin(rlon.0);
    let coslon = f32::cos(rlon.0);
    (
        altitude * coslat * coslon,
        altitude * coslat * sinlon,
        altitude * sinlat,
    )
}

pub fn mapidx<N: Into<usize>>(x:N, y:N, z:N) -> usize {
    use crate::planet::{REGION_HEIGHT, REGION_WIDTH};
    (z.into() * REGION_HEIGHT as usize * REGION_WIDTH as usize) + (y.into() * REGION_WIDTH as usize) + x.into()
}

pub fn idxmap(idx: usize) -> (usize, usize, usize) {
    use crate::planet::{REGION_HEIGHT, REGION_WIDTH};
    const LAYER_SIZE : usize = REGION_WIDTH as usize * REGION_HEIGHT as usize;
    let z = idx / LAYER_SIZE;
    let mut tidx = idx - (z * LAYER_SIZE);

    let y = tidx / REGION_WIDTH as usize;
    tidx -= y * REGION_WIDTH as usize;

    let x = tidx;
    (x, y, z)
}