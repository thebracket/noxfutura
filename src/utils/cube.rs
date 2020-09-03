pub fn add_cube_geometry(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    d: f32,
    material_index: usize,
) {
    let mi = material_index as f32;
    let x0 = x;
    let x1 = x0 + w;
    let y0 = z;
    let y1 = y0 + d;
    let z0 = y;
    let z1 = z0 + h;

    #[rustfmt::skip]
    let cube_geometry = [
        x0, y0, z0,    1.0,  mi,
        x1, y1, z0,    1.0,  mi,
        x1, y0, z0,    1.0,  mi,
        x1, y1, z0,    1.0,  mi,
        x0, y0, z0,    1.0,  mi,
        x0, y1, z0,    1.0,  mi,

        x0, y0, z1,    2.0,   mi,
        x1, y0, z1,    2.0,   mi,
        x1, y1, z1,    2.0,   mi,
        x1, y1, z1,    2.0,   mi,
        x0, y1, z1,    2.0,   mi,
        x0, y0, z1,    2.0,   mi,

        x0, y1, z1,    3.0,   mi,
        x0, y1, z0,    3.0,   mi,
        x0, y0, z0,    3.0,   mi,
        x0, y0, z0,    3.0,   mi,
        x0, y0, z1,    3.0,   mi,
        x0, y1, z1,    3.0,   mi,

        x1, y1, z1,    4.0,  mi,
        x1, y0, z0,    4.0,  mi,
        x1, y1, z0,    4.0,  mi,
        x1, y0, z0,    4.0,  mi,
        x1, y1, z1,    4.0,  mi,
        x1, y0, z1,    4.0,  mi,

        x0, y0, z0,   5.0,   mi,
        x1, y0, z0,   5.0,   mi,
        x1, y0, z1,   5.0,   mi,
        x1, y0, z1,   5.0,   mi,
        x0, y0, z1,   5.0,   mi,
        x0, y0, z0,   5.0,   mi,

        x1, y1, z1,   0.0,    mi,
        x1, y1, z0,   0.0,    mi,
        x0, y1, z0,   0.0,    mi,
        x0, y1, z0,   0.0,    mi,
        x0, y1, z1,   0.0,    mi,
        x1, y1, z1,   0.0,    mi,
    ];
    vb.extend_from_slice(&cube_geometry);
    *element_count += 12;
}
