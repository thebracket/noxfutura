const GEOMETRY_SIZE: f32 = 1.0;

const CUBE_NORMALS: [[f32;3];6] = [
    [0.0, 0.0, -1.0],
    [0.0, 0.0, 1.0],
    [-1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, 1.0, 0.0],
];

const CUBE_TANGENTS: [[f32;3];6] = [
    [-1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, -1.0],
    [1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
];

pub fn add_cube_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    d: f32,
) {
    let x0 = x * GEOMETRY_SIZE;
    let x1 = (x0 + w) * GEOMETRY_SIZE;
    let y0 = y * GEOMETRY_SIZE;
    let y1 = (y0 + h) * GEOMETRY_SIZE;
    let z0 = z * GEOMETRY_SIZE;
    let z1 = (z0 + d) * GEOMETRY_SIZE;

    //println!("Cube at: {},{},{}", x0, y0, z0);

    #[rustfmt::skip]
    let cube_geometry = [
        [x0, y0, z0,],
        [x1, y1, z0,],
        [x1, y0, z0,],
        [x1, y1, z0,],
        [x0, y0, z0,],
        [x0, y1, z0,],

        [x0, y0, z1,],
        [x1, y0, z1,],
        [x1, y1, z1,],
        [x1, y1, z1,],
        [x0, y1, z1,],
        [x0, y0, z1,],

        [x0, y1, z1,],
        [x0, y1, z0,],
        [x0, y0, z0,],
        [x0, y0, z0,],
        [x0, y0, z1,],
        [x0, y1, z1,],

        [x1, y1, z1,],
        [x1, y0, z0,],
        [x1, y1, z0,],
        [x1, y0, z0,],
        [x1, y1, z1,],
        [x1, y0, z1,],

        [x0, y0, z0,],
        [x1, y0, z0,],
        [x1, y0, z1,],
        [x1, y0, z1,],
        [x0, y0, z1,],
        [x0, y0, z0,],

        [x1, y1, z1,],
        [x1, y1, z0,],
        [x0, y1, z0,],
        [x0, y1, z0,],
        [x0, y1, z1,],
        [x1, y1, z1,],
    ];
    vertices.extend_from_slice(&cube_geometry);

    #[rustfmt::skip]
    const NORMAL_GEOMETRY: [[f32; 3]; 36] = [
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],

        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],

        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],

        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],

        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],

        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
    ];
    normals.extend_from_slice(&NORMAL_GEOMETRY);

    #[rustfmt::skip]
    const TANGENT_GEOMETRY: [[f32; 3]; 36] = [
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],

        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],

        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],

        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],

        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],

        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
    ];
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

    let tw = w;
    let th = h;
    #[rustfmt::skip]
    let uv_base: [[f32; 2]; 36] = [
        [0.0, 0.0],
        [tw, th],
        [tw, 0.0],
        [tw, th],
        [0.0, 0.0],
        [0.0, th],

        [0.0, 0.0],
        [tw, 0.0],
        [tw, th],
        [tw, th],
        [0.0, th],
        [0.0, 0.0],

        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],

        [tw, th],
        [0.0, 0.0],
        [tw, 0.0],
        [0.0, 0.0],
        [tw, th],
        [0.0, th],

        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],

        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],
    ];

    uv.extend_from_slice(&uv_base);
}
