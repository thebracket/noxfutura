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
    use crate::planet::{REGION_HEIGHT, REGION_WIDTH, REGION_DEPTH};
    let xc = x.into();
    let yc = y.into();
    let zc = z.into();
    debug_assert!(xc <=REGION_WIDTH && yc <=REGION_HEIGHT && zc < REGION_DEPTH);
    (zc * REGION_HEIGHT as usize * REGION_WIDTH as usize) + (yc * REGION_WIDTH as usize) + xc
}

pub fn idxmap(mut idx: usize) -> (usize, usize, usize) {
    use crate::planet::{REGION_HEIGHT, REGION_WIDTH, REGION_DEPTH};
    debug_assert!(idx < REGION_DEPTH * REGION_WIDTH * REGION_HEIGHT);
    const LAYER_SIZE : usize = REGION_WIDTH as usize * REGION_HEIGHT as usize;
    let z = idx / LAYER_SIZE;
    idx -= z * LAYER_SIZE;

    let y = idx / REGION_WIDTH as usize;
    idx -= y * REGION_WIDTH as usize;

    let x = idx;
    debug_assert!(x <=REGION_WIDTH && y <=REGION_HEIGHT && z < REGION_DEPTH);
    (x, y, z)
}

use crate::engine::VertexBuffer;

pub fn add_cube_geometry(
    vb: &mut VertexBuffer<f32>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    d: f32
)
{
    use crate::planet::*;

    let x0 = (-0.5 + x) - (REGION_WIDTH as f32 / 2.0);
    let x1 = x0 + w - (REGION_WIDTH as f32 / 2.0);;
    let y0= -0.5 + z - (REGION_HEIGHT as f32 / 2.0);;
    let y1 = y0 + d - (REGION_HEIGHT as f32 / 2.0);;
    let z0 = 0.5 + y - (REGION_DEPTH as f32 / 2.0);;
    let z1 = z0 + h - (REGION_DEPTH as f32 / 2.0);;

    let green = z / 256.0;
    let red = x / 256.0;
    //println!("{}", green);

    let cube_geometry = [
        x0, y0, z0, red, green, 0.0, 1.0,
        x1, y1, z0, red, green, 0.0, 1.0,
        x1, y0, z0, red, green, 0.0, 1.0,
        x1, y1, z0, red, green, 0.0, 1.0,
        x0, y0, z0, red, green, 0.0, 1.0,
        x0, y1, z0, red, green, 0.0, 1.0,

        x0, y0, z1, red, green, 0.0, 1.0,
        x1, y0, z1, red, green, 0.0, 1.0,
        x1, y1, z1, red, green, 0.0, 1.0,
        x1, y1, z1, red, green, 0.0, 1.0,
        x0, y1, z1, red, green, 0.0, 1.0,
        x0, y0, z1, red, green, 0.0, 1.0,

        x0, y1, z1, red, green, 0.0, 1.0,
        x0, y1, z0, red, green, 0.0, 1.0,
        x0, y0, z0, red, green, 0.0, 1.0,
        x0, y0, z0, red, green, 0.0, 1.0,
        x0, y0, z1, red, green, 0.0, 1.0,
        x0, y1, z1, red, green, 0.0, 1.0,

        x1, y1, z1, red, green, 0.0, 1.0,
        x1, y0, z0, red, green, 0.0, 1.0,
        x1, y1, z0, red, green, 0.0, 1.0,
        x1, y0, z0, red, green, 0.0, 1.0,
        x1, y1, z1, red, green, 0.0, 1.0,
        x1, y0, z1, red, green, 0.0, 1.0,

        x0, y0, z0, red, green, 0.0, 1.0,
        x1, y0, z0, red, green, 0.0, 1.0,
        x1, y0, z1, red, green, 0.0, 1.0,
        x1, y0, z1, red, green, 0.0, 1.0,
        x0, y0, z1, red, green, 0.0, 1.0,
        x0, y0, z0, red, green, 0.0, 1.0,

        x1, y1, z1, red, green, 0.0, 1.0,
        x1, y1, z0, red, green, 0.0, 1.0,
        x0, y1, z0, red, green, 0.0, 1.0,
        x0, y1, z0, red, green, 0.0, 1.0,
        x0, y1, z1, red, green, 0.0, 1.0,
        x1, y1, z1, red, green, 0.0, 1.0,
    ];
    vb.add_slice(&cube_geometry);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapidx_idxmap() {
        let (x,y,z) = (12, 19, 11);
        let idx = mapidx(x, y, z);
        let (nx, ny, nz) = idxmap(idx);
        assert_eq!(x, nx);
        assert_eq!(y, ny);
        assert_eq!(z, nz);
    }

    #[test]
    fn test_mapidx() {
        assert_eq!(mapidx(1usize, 0usize, 0usize), 1usize);
        assert_eq!(mapidx(2usize, 0usize, 0usize), 2usize);
    }
}