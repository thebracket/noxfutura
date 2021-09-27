use super::constants::*;

pub fn add_floor_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    let x0 = x * GEOMETRY_SIZE;
    let x1 = (x0 + w) * GEOMETRY_SIZE;
    let y0 = y * GEOMETRY_SIZE;
    let y1 = (y0 + h) * GEOMETRY_SIZE;
    let z0 = z * GEOMETRY_SIZE;

    //println!("Cube at: {},{},{}", x0, y0, z0);

    #[rustfmt::skip]
    let cube_geometry = [
        [x0, y0, z0,],
        [x1, y0, z0,],
        [x1, y1, z0,],
        [x1, y1, z0,],
        [x0, y1, z0,],
        [x0, y0, z0,],
    ];
    vertices.extend_from_slice(&cube_geometry);

    #[rustfmt::skip]
    const NORMAL_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
    ];
    normals.extend_from_slice(&NORMAL_GEOMETRY);

    #[rustfmt::skip]
    const TANGENT_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
    ];
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

    let tw = w;
    let th = h;
    #[rustfmt::skip]
    let uv_base: [[f32; 2]; 6] = [
        [0.0, 0.0],
        [tw, 0.0],
        [tw, th],
        [tw, th],
        [0.0, th],
        [0.0, 0.0],
    ];

    uv.extend_from_slice(&uv_base);
}
