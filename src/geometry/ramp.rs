use super::constants::*;
use crate::simulation::{idxmap, terrain::RampDirection};

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
    let z = z as f32;
    match direction {
        RampDirection::NorthSouth => north_south(x, y, z, vertices, normals, uv, tangents),
        RampDirection::SouthNorth => south_north(x, y, z, vertices, normals, uv, tangents),
        RampDirection::EastWest => east_west(x, y, z, vertices, normals, uv, tangents),
        RampDirection::WestEast => west_east(x, y, z, vertices, normals, uv, tangents),
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

    let normal_geometry = [
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[6],
        CUBE_NORMALS[6],
        CUBE_NORMALS[6],
        CUBE_NORMALS[6],
        CUBE_NORMALS[6],
        CUBE_NORMALS[6],
    ];
    normals.extend_from_slice(&normal_geometry);

    let tangent_geometry = [
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[6],
        CUBE_TANGENTS[6],
        CUBE_TANGENTS[6],
        CUBE_TANGENTS[6],
        CUBE_TANGENTS[6],
        CUBE_TANGENTS[6],
    ];
    tangents.extend_from_slice(&tangent_geometry);

    let uv_geometry = [
        [t0, t0],
        [tw, th],
        [tw, t0],
        [tw, th],
        [t0, t0],
        [t0, th],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [t0, t0],
        [t0, th],
        [tw, th],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [t0, t0],
        [t0, th],
        [tw, th],
    ];
    uv.extend_from_slice(&uv_geometry);
}

fn south_north(
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

    let cube_geometry = [
        [x0, y0, z1],
        [x1, y0, z1],
        [x1, y1, z1],
        [x1, y1, z1],
        [x0, y1, z1],
        [x0, y0, z1],
        [x0, y0, z0],
        [x0, y0, z1],
        [x0, y1, z1],
        [x1, y0, z0],
        [x1, y1, z1],
        [x1, y0, z1],
        [x0, y0, z0],
        [x1, y0, z0],
        [x1, y0, z1],
        [x1, y0, z1],
        [x0, y0, z1],
        [x0, y0, z0],
        [x1, y1, z1],
        [x1, y0, z0],
        [x0, y0, z0],
        [x0, y0, z0],
        [x0, y1, z1],
        [x1, y1, z1],
    ];
    vertices.extend_from_slice(&cube_geometry);

    let normal_geometry = [
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[7],
        CUBE_NORMALS[7],
        CUBE_NORMALS[7],
        CUBE_NORMALS[7],
        CUBE_NORMALS[7],
        CUBE_NORMALS[7],
    ];
    normals.extend_from_slice(&normal_geometry);

    let tangent_geometry = [
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[7],
        CUBE_TANGENTS[7],
        CUBE_TANGENTS[7],
        CUBE_TANGENTS[7],
        CUBE_TANGENTS[7],
        CUBE_TANGENTS[7],
    ];
    tangents.extend_from_slice(&tangent_geometry);

    let uv_geometry = [
        [t0, t0],
        [tw, t0],
        [tw, th],
        [tw, th],
        [t0, th],
        [t0, t0],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [tw, th],
        [t0, t0],
        [tw, t0],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [t0, t0],
        [t0, th],
        [tw, th],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [t0, t0],
        [t0, th],
        [tw, th],
    ];
    uv.extend_from_slice(&uv_geometry);
}

fn east_west(
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

    let cube_geometry = [
        [x1, y0, z0],
        [x0, y0, z0],
        [x0, y1, z0],
        [x1, y0, z1],
        [x0, y0, z1],
        [x0, y1, z1],
        [x0, y1, z1],
        [x0, y1, z0],
        [x0, y0, z0],
        [x0, y0, z0],
        [x0, y0, z1],
        [x0, y1, z1],
        [x0, y0, z0],
        [x1, y0, z0],
        [x1, y0, z1],
        [x1, y0, z1],
        [x0, y0, z1],
        [x0, y0, z0],
        [x1, y0, z1],
        [x1, y0, z0],
        [x0, y1, z0],
        [x0, y1, z0],
        [x0, y1, z1],
        [x1, y0, z1],
    ];
    vertices.extend_from_slice(&cube_geometry);

    let normal_geometry = [
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[8],
        CUBE_NORMALS[8],
        CUBE_NORMALS[8],
        CUBE_NORMALS[8],
        CUBE_NORMALS[8],
        CUBE_NORMALS[8],
    ];
    normals.extend_from_slice(&normal_geometry);

    let tangent_geometry = [
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[8],
        CUBE_TANGENTS[8],
        CUBE_TANGENTS[8],
        CUBE_TANGENTS[8],
        CUBE_TANGENTS[8],
        CUBE_TANGENTS[8],
    ];
    tangents.extend_from_slice(&tangent_geometry);

    let uv_geometry = [
        [t0, t0],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [tw, t0],
        [tw, th],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [t0, t0],
        [t0, th],
        [tw, th],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [t0, t0],
        [t0, th],
        [tw, th],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [t0, t0],
        [t0, th],
        [tw, th],
    ];
    uv.extend_from_slice(&uv_geometry);
}

fn west_east(
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

    let cube_geometry = [
        [x0, y0, z0],
        [x1, y1, z0],
        [x1, y0, z0],
        [x0, y0, z1],
        [x1, y0, z1],
        [x1, y1, z1],
        [x1, y1, z1],
        [x1, y0, z0],
        [x1, y1, z0],
        [x1, y0, z0],
        [x1, y1, z1],
        [x1, y0, z1],
        [x0, y0, z0],
        [x1, y0, z0],
        [x1, y0, z1],
        [x1, y0, z1],
        [x0, y0, z1],
        [x0, y0, z0],
        [x1, y1, z1],
        [x1, y1, z0],
        [x0, y0, z0],
        [x0, y0, z0],
        [x0, y0, z1],
        [x1, y1, z1],
    ];
    vertices.extend_from_slice(&cube_geometry);

    let normal_geometry = [
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[9],
        CUBE_NORMALS[9],
        CUBE_NORMALS[9],
        CUBE_NORMALS[9],
        CUBE_NORMALS[9],
        CUBE_NORMALS[9],
    ];
    normals.extend_from_slice(&normal_geometry);

    let tangent_geometry = [
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[9],
        CUBE_TANGENTS[9],
        CUBE_TANGENTS[9],
        CUBE_TANGENTS[9],
        CUBE_TANGENTS[9],
        CUBE_TANGENTS[9],
    ];
    tangents.extend_from_slice(&tangent_geometry);

    let uv_geometry = [
        [t0, t0],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [tw, t0],
        [tw, th],
        [tw, th],
        [t0, t0],
        [tw, t0],
        [t0, t0],
        [tw, th],
        [t0, th],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [t0, t0],
        [t0, th],
        [tw, th],
        [tw, th],
        [tw, t0],
        [t0, t0],
        [t0, t0],
        [t0, th],
        [tw, th],
    ];
    uv.extend_from_slice(&uv_geometry);
}
