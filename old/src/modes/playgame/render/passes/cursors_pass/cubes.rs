pub fn add_cube_geometry(
    vb: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    d: f32,
    mi: f32,
) {
    let x0 = x;
    let x1 = x0 + w;
    let y0 = z;
    let y1 = y0 + d;
    let z0 = y;
    let z1 = z0 + h;

    let t0 = 0.0f32;
    let tw = w;
    let th = h;

    #[rustfmt::skip]
    let cube_geometry = [
        x0, y0, z0, t0, t0, mi,
        x1, y1, z0, tw, th, mi,
        x1, y0, z0, tw, t0, mi,
        x1, y1, z0, tw, th, mi,
        x0, y0, z0, t0, t0, mi,
        x0, y1, z0, t0, th, mi,

        x0, y0, z1, t0, t0, mi,
        x1, y0, z1, tw, t0, mi,
        x1, y1, z1, tw, th, mi,
        x1, y1, z1, tw, th, mi,
        x0, y1, z1, t0, th, mi,
        x0, y0, z1, t0, t0, mi,

        x0, y1, z1,  tw, th, mi,
        x0, y1, z0,  tw, t0, mi,
        x0, y0, z0,  t0, t0, mi,
        x0, y0, z0,  t0, t0, mi,
        x0, y0, z1,  t0, th, mi,
        x0, y1, z1,  tw, th, mi,

        x1, y1, z1, tw, th, mi,
        x1, y0, z0, t0, t0, mi,
        x1, y1, z0, tw, t0, mi,
        x1, y0, z0, t0, t0, mi,
        x1, y1, z1, tw, th, mi,
        x1, y0, z1, t0, th, mi,

        x0, y0, z0, tw, th, mi,
        x1, y0, z0, tw, t0, mi,
        x1, y0, z1, t0, t0, mi,
        x1, y0, z1, t0, t0, mi,
        x0, y0, z1, t0, th, mi,
        x0, y0, z0, tw, th, mi,

        x1, y1, z1, tw, th, mi,
        x1, y1, z0, tw, t0, mi,
        x0, y1, z0, t0, t0, mi,
        x0, y1, z0, t0, t0, mi,
        x0, y1, z1, t0, th, mi,
        x1, y1, z1, tw, th, mi,
    ];
    vb.extend_from_slice(&cube_geometry);
}
