/*use crate::simulation::{idxmap, terrain::chunker::RampDirection};

pub fn add_ramp_geometry(
    direction: RampDirection,
    idx: usize,
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
) {
    let (x, y, z) = idxmap(idx);
    let x = x as f32;
    let y = y as f32;
    let z  = z as f32;
    match direction {
        RampDirection::NorthSouth => north_south(x, y, z, vertices, normals, uv, tangents),
        /*RampDirection::SouthNorth => south_north(vb, element_count, x, y, z, mi, tex),
        RampDirection::EastWest => east_west(vb, element_count, x, y, z, mi, tex),
        RampDirection::WestEast => west_east(vb, element_count, x, y, z, mi, tex), */
        _ => {}
    }
}

fn north_south(
    x: f32,
    y: f32,
    z: f32,
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
) {
    let w = 1.0;
    let h = 1.0;
    let d = 1.0;

    let x0 = x;
    let x1 = x0 + w;
    let y0 = y;
    let y1 = y0 + h;
    let z0 = z;
    let z1 = z0 + d;

    let t0 = 0.0f32;
    let tw = w;
    let th = h;

    #[rustfmt::skip]
    let cube_geometry = [
        [x0, y0, z0],
        [x1, y1, z0],
        [x1, y0, z0],
        [x1, y1, z0],
        [x0, y0, z0],
        [x0, y1, z0],

        [x0, y0, z0],
        [x0, y1, z0],
        [x0, y0, z1],

        [x1, y0, z0],
        [x1, y1, z0],
        [x1, y0, z1],

        [x0, y0, z0],
        [x1, y0, z0],
        [x1, y0, z1],
        [x1, y0, z1],
        [x0, y0, z1],
        [x0, y0, z0],

        [x1, y0, z1],
        [x1, y1, z0],
        [x0, y1, z0],
        [x0, y1, z0],
        [x0, y0, z1],
        [x1, y0, z1],
    ];
    vertices.extend_from_slice(&cube_geometry);
}
*/
