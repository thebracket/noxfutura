use bevy::math::Vec3;

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
        [x1, y1, z0],
        [x0, y0, z0],
        [x0, y1, z0],
        [x0, y0, z0],
        [x1, y1, z0],
        [x1, y0, z0],
        [x1, y1, z0],
        [x1, y0, z0],
        [x1, y1, z1],
        [x0, y1, z0],
        [x0, y0, z0],
        [x0, y1, z1],
        [x1, y1, z0],
        [x0, y1, z0],
        [x0, y1, z1],
        [x0, y1, z1],
        [x1, y1, z1],
        [x1, y1, z0],
        [x0, y1, z1],
        [x0, y0, z0],
        [x1, y0, z0],
        [x1, y0, z0],
        [x1, y1, z1],
        [x0, y1, z1],
    ];
    vertices.extend_from_slice(&cube_geometry);

    const NORMAL_GEOMETRY: [[f32; 3]; 24] = [
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -0.5, 0.5],
        [0.0, -0.5, 0.5],
        [0.0, -0.5, 0.5],
        [0.0, -0.5, 0.5],
        [0.0, -0.5, 0.5],
        [0.0, -0.5, 0.5],
    ];
    normals.extend_from_slice(&NORMAL_GEOMETRY);

    const TANGENT_GEOMETRY: [[f32; 3]; 24] = [
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, -0.0, -1.0],
        [0.0, -0.0, -1.0],
        [0.0, -0.0, -1.0],
        [0.0, -0.0, -1.0],
        [0.0, -0.0, -1.0],
        [0.0, -0.0, -1.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, -0.5, 0.0],
        [0.0, -0.5, 0.0],
        [0.0, -0.5, 0.0],
        [0.0, -0.5, 0.0],
        [0.0, -0.5, 0.0],
        [0.0, -0.5, 0.0],
    ];
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

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
        [x1, y0, z0],
        [x0, y1, z0],
        [x1, y1, z0],
        [x0, y1, z0],
        [x1, y0, z0],
        [x0, y0, z0],
        [x1, y0, z0],
        [x0, y0, z0],
        [x1, y0, z1],
        [x1, y1, z0],
        [x0, y1, z0],
        [x1, y1, z1],
        [x1, y0, z0],
        [x1, y1, z0],
        [x1, y1, z1],
        [x1, y1, z1],
        [x1, y0, z1],
        [x1, y0, z0],
        [x1, y1, z1],
        [x0, y1, z0],
        [x0, y0, z0],
        [x0, y0, z0],
        [x1, y0, z1],
        [x1, y1, z1],
    ];

    const NORMAL_GEOMETRY: [[f32; 3]; 24] = [
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-0.0, 1.0, 0.0],
        [-0.0, 1.0, 0.0],
        [-0.0, 1.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [-0.5, -0.0, 0.5],
        [-0.5, -0.0, 0.5],
        [-0.5, -0.0, 0.5],
        [-0.5, -0.0, 0.5],
        [-0.5, -0.0, 0.5],
        [-0.5, -0.0, 0.5],
    ];
    const TANGENT_GEOMETRY: [[f32; 3]; 24] = [
        [-0.0, 1.0, 0.0],
        [-0.0, 1.0, 0.0],
        [-0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [-0.0, 0.0, -1.0],
        [-0.0, 0.0, -1.0],
        [-0.0, 0.0, -1.0],
        [-0.0, 0.0, -1.0],
        [-0.0, 0.0, -1.0],
        [-0.0, 0.0, -1.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [-0.5, -0.0, 0.0],
        [-0.5, -0.0, 0.0],
        [-0.5, -0.0, 0.0],
        [-0.5, -0.0, 0.0],
        [-0.5, -0.0, 0.0],
        [-0.5, -0.0, 0.0],
    ];
    vertices.extend_from_slice(&cube_geometry);
    normals.extend_from_slice(&NORMAL_GEOMETRY);
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

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
        [x0, y1, z0],
        [x1, y0, z0],
        [x0, y0, z0],
        [x1, y0, z0],
        [x0, y1, z0],
        [x1, y1, z0],
        [x0, y1, z0],
        [x1, y1, z0],
        [x0, y1, z1],
        [x0, y0, z0],
        [x1, y0, z0],
        [x0, y0, z1],
        [x0, y1, z0],
        [x0, y0, z0],
        [x0, y0, z1],
        [x0, y0, z1],
        [x0, y1, z1],
        [x0, y1, z0],
        [x0, y0, z1],
        [x1, y0, z0],
        [x1, y1, z0],
        [x1, y1, z0],
        [x0, y1, z1],
        [x0, y0, z1],
    ];

    const NORMAL_GEOMETRY: [[f32; 3]; 24] = [
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.5, 0.0, 0.5],
        [0.5, 0.0, 0.5],
        [0.5, 0.0, 0.5],
        [0.5, 0.0, 0.5],
        [0.5, 0.0, 0.5],
        [0.5, 0.0, 0.5],
    ];
    const TANGENT_GEOMETRY: [[f32; 3]; 24] = [
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [-0.0, 1.0, 0.0],
        [-0.0, 1.0, 0.0],
        [-0.0, 1.0, 0.0],
        [-0.0, 1.0, 0.0],
        [-0.0, 1.0, 0.0],
        [-0.0, 1.0, 0.0],
        [0.5, 0.0, 0.0],
        [0.5, 0.0, 0.0],
        [0.5, 0.0, 0.0],
        [0.5, 0.0, 0.0],
        [0.5, 0.0, 0.0],
        [0.5, 0.0, 0.0],
    ];
    vertices.extend_from_slice(&cube_geometry);
    normals.extend_from_slice(&NORMAL_GEOMETRY);
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

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

/// This function should not be used, it generates the constants
/// in the ramp functions because I suck at 3D rotation in my head.
#[allow(dead_code)]
pub fn ramp_rotation_helper() {
    let x0 = -0.5;
    let x1 = 0.5;
    let y0 = -0.5;
    let y1 = 0.5;
    let z0 = -0.5;
    let z1 = 0.5;

    let cube_geometry: [Vec3; 24] = [
        [x0, y0, z0].into(),
        [x1, y1, z0].into(),
        [x1, y0, z0].into(),
        [x1, y1, z0].into(),
        [x0, y0, z0].into(),
        [x0, y1, z0].into(),
        [x0, y0, z0].into(),
        [x0, y1, z0].into(),
        [x0, y0, z1].into(),
        [x1, y0, z0].into(),
        [x1, y1, z0].into(),
        [x1, y0, z1].into(),
        [x0, y0, z0].into(),
        [x1, y0, z0].into(),
        [x1, y0, z1].into(),
        [x1, y0, z1].into(),
        [x0, y0, z1].into(),
        [x0, y0, z0].into(),
        [x1, y0, z1].into(),
        [x1, y1, z0].into(),
        [x0, y1, z0].into(),
        [x0, y1, z0].into(),
        [x0, y0, z1].into(),
        [x1, y0, z1].into(),
    ];

    let normal_geometry: [Vec3; 24] = [
        CUBE_NORMALS[1].into(),
        CUBE_NORMALS[1].into(),
        CUBE_NORMALS[1].into(),
        CUBE_NORMALS[1].into(),
        CUBE_NORMALS[1].into(),
        CUBE_NORMALS[1].into(),
        CUBE_NORMALS[5].into(),
        CUBE_NORMALS[5].into(),
        CUBE_NORMALS[5].into(),
        CUBE_NORMALS[3].into(),
        CUBE_NORMALS[3].into(),
        CUBE_NORMALS[3].into(),
        CUBE_NORMALS[5].into(),
        CUBE_NORMALS[5].into(),
        CUBE_NORMALS[5].into(),
        CUBE_NORMALS[5].into(),
        CUBE_NORMALS[5].into(),
        CUBE_NORMALS[5].into(),
        CUBE_NORMALS[6].into(),
        CUBE_NORMALS[6].into(),
        CUBE_NORMALS[6].into(),
        CUBE_NORMALS[6].into(),
        CUBE_NORMALS[6].into(),
        CUBE_NORMALS[6].into(),
    ];

    let tangent_geometry: [Vec3; 24] = [
        CUBE_TANGENTS[1].into(),
        CUBE_TANGENTS[1].into(),
        CUBE_TANGENTS[1].into(),
        CUBE_TANGENTS[2].into(),
        CUBE_TANGENTS[2].into(),
        CUBE_TANGENTS[2].into(),
        CUBE_TANGENTS[3].into(),
        CUBE_TANGENTS[3].into(),
        CUBE_TANGENTS[3].into(),
        CUBE_TANGENTS[3].into(),
        CUBE_TANGENTS[3].into(),
        CUBE_TANGENTS[3].into(),
        CUBE_TANGENTS[5].into(),
        CUBE_TANGENTS[5].into(),
        CUBE_TANGENTS[5].into(),
        CUBE_TANGENTS[5].into(),
        CUBE_TANGENTS[5].into(),
        CUBE_TANGENTS[5].into(),
        CUBE_TANGENTS[9].into(),
        CUBE_TANGENTS[9].into(),
        CUBE_TANGENTS[9].into(),
        CUBE_TANGENTS[9].into(),
        CUBE_TANGENTS[9].into(),
        CUBE_TANGENTS[9].into(),
    ];

    let rotator = bevy::math::Quat::from_rotation_z(90.0 * 0.0174533);
    println!("let cube_geometry = [");
    cube_geometry
        .iter()
        .map(|v| rotator.mul_vec3(*v))
        .for_each(|v| {
            print!("   [");
            if v.x < 0.0 {
                print!("x0, ");
            } else {
                print!("x1, ")
            };
            if v.y < 0.0 {
                print!("y0, ");
            } else {
                print!("y1, ")
            };
            if v.z < 0.0 {
                print!("z0");
            } else {
                print!("z1")
            };
            println!("],");
        });
    println!("];");
    println!("");

    println!("const NORMAL_GEOMETRY : [[f32;3];24] = [");
    normal_geometry
        .iter()
        .map(|v| rotator.mul_vec3(*v))
        .for_each(|v| {
            print!("   [");
            print!("{:.1}, {:.1}, {:.1}", v.x, v.y, v.z);
            println!("],");
        });
    println!("];");

    println!("const TANGENT_GEOMETRY : [[f32;3];24] = [");
    tangent_geometry
        .iter()
        .map(|v| rotator.mul_vec3(*v))
        .for_each(|v| {
            print!("   [");
            print!("{:.1}, {:.1}, {:.1}", v.x, v.y, v.z);
            println!("],");
        });
    println!("];");

    std::process::exit(1);
}
