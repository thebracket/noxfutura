use crate::opengl::*;

pub fn add_cube_geometry(
    vb: &mut VertexArray,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    d: f32
)
{
    let x0 = -0.5 + x;
    let x1 = x0 + w;
    let y0= -0.5 + z;
    let y1 = y0 + d;
    let z0 = 0.5 + y;
    let z1 = z0 + h;

    let green = z / 256.0;

    let cube_geometry = [
        x0, y0, z0, 0.0, green, 0.0, 1.0,
        x1, y1, z0, 0.0, green, 0.0, 1.0,
        x1, y0, z0, 0.0, green, 0.0, 1.0,
        x1, y1, z0, 0.0, green, 0.0, 1.0,
        x0, y0, z0, 0.0, green, 0.0, 1.0,
        x0, y1, z0, 0.0, green, 0.0, 1.0,

        x0, y0, z1, 0.0, green, 0.0, 1.0,
        x1, y0, z1, 0.0, green, 0.0, 1.0,
        x1, y1, z1, 0.0, green, 0.0, 1.0,
        x1, y1, z1, 0.0, green, 0.0, 1.0,
        x0, y1, z1, 0.0, green, 0.0, 1.0,
        x0, y0, z1, 0.0, green, 0.0, 1.0,

        x0, y1, z1, 0.0, green, 0.0, 1.0,
        x0, y1, z0, 0.0, green, 0.0, 1.0,
        x0, y0, z0, 0.0, green, 0.0, 1.0,
        x0, y0, z0, 0.0, green, 0.0, 1.0,
        x0, y0, z1, 0.0, green, 0.0, 1.0,
        x0, y1, z1, 0.0, green, 0.0, 1.0,

        x1, y1, z1, 0.0, green, 0.0, 1.0,
        x1, y0, z0, 0.0, green, 0.0, 1.0,
        x1, y1, z0, 0.0, green, 0.0, 1.0,
        x1, y0, z0, 0.0, green, 0.0, 1.0,
        x1, y1, z1, 0.0, green, 0.0, 1.0,
        x1, y0, z1, 0.0, green, 0.0, 1.0,

        x0, y0, z0, 0.0, green, 0.0, 1.0,
        x1, y0, z0, 0.0, green, 0.0, 1.0,
        x1, y0, z1, 0.0, green, 0.0, 1.0,
        x1, y0, z1, 0.0, green, 0.0, 1.0,
        x0, y0, z1, 0.0, green, 0.0, 1.0,
        x0, y0, z0, 0.0, green, 0.0, 1.0,

        x1, y1, z1, 0.0, green, 0.0, 1.0,
        x1, y1, z0, 0.0, green, 0.0, 1.0,
        x0, y1, z0, 0.0, green, 0.0, 1.0,
        x0, y1, z0, 0.0, green, 0.0, 1.0,
        x0, y1, z1, 0.0, green, 0.0, 1.0,
        x1, y1, z1, 0.0, green, 0.0, 1.0,
    ];
    vb.add_slice(&cube_geometry);
}