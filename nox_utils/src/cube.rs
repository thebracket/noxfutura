use nox_raws::RAWS;

pub fn add_cube_geometry(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    d: f32,
    material_index: (usize, bool),
) {
    let rlock = RAWS.read();
    let mi = *rlock.matmap.get(material_index.0) as f32;
    let tex = if material_index.1 {
        rlock.materials.material_list.as_ref().unwrap()[material_index.0].constructed
    } else {
        rlock.materials.material_list.as_ref().unwrap()[material_index.0].base
    };

    //let mi = material_index as f32;
    //let tex = -1.0;
    let x0 = x;
    let x1 = x0 + w;
    let y0 = z;
    let y1 = y0 + d - 0.1;
    let z0 = y;
    let z1 = z0 + h;

    let t0 = 0.0f32;
    let tw = w;
    let th = h;

    #[rustfmt::skip]
    let cube_geometry = [
        x0, y0, z0,   t0, t0,  1.0,  mi, tex,
        x1, y1, z0,   tw, th,  1.0,  mi, tex,
        x1, y0, z0,   tw, t0,  1.0,  mi, tex,
        x1, y1, z0,   tw, th,  1.0,  mi, tex,
        x0, y0, z0,   t0, t0,  1.0,  mi, tex,
        x0, y1, z0,   t0, th,  1.0,  mi, tex,

        x0, y0, z1,    t0, t0,  2.0,   mi, tex,
        x1, y0, z1,    tw, t0,  2.0,   mi, tex,
        x1, y1, z1,    tw, th,  2.0,   mi, tex,
        x1, y1, z1,    tw, th,  2.0,   mi, tex,
        x0, y1, z1,    t0, th,  2.0,   mi, tex,
        x0, y0, z1,    t0, t0,  2.0,   mi, tex,

        x0, y1, z1,    tw, th,  3.0,   mi, tex,
        x0, y1, z0,    tw, t0,  3.0,   mi, tex,
        x0, y0, z0,    t0, t0,  3.0,   mi, tex,
        x0, y0, z0,    t0, t0,  3.0,   mi, tex,
        x0, y0, z1,    t0, th,  3.0,   mi, tex,
        x0, y1, z1,    tw, th,  3.0,   mi, tex,

        x1, y1, z1,   tw, th,  4.0,  mi, tex,
        x1, y0, z0,   t0, t0,  4.0,  mi, tex,
        x1, y1, z0,   tw, t0,  4.0,  mi, tex,
        x1, y0, z0,   t0, t0,  4.0,  mi, tex,
        x1, y1, z1,   tw, th,  4.0,  mi, tex,
        x1, y0, z1,   t0, th,  4.0,  mi, tex,

        x0, y0, z0,  tw, th,  5.0,   mi, tex,
        x1, y0, z0,  tw, t0,  5.0,   mi, tex,
        x1, y0, z1,  t0, t0,  5.0,   mi, tex,
        x1, y0, z1,  t0, t0,  5.0,   mi, tex,
        x0, y0, z1,  t0, th,  5.0,   mi, tex,
        x0, y0, z0,  tw, th,  5.0,   mi, tex,

        x1, y1, z1,   tw, th, 0.0,    mi, tex,
        x1, y1, z0,   tw, t0, 0.0,    mi, tex,
        x0, y1, z0,   t0, t0, 0.0,    mi, tex,
        x0, y1, z0,   t0, t0, 0.0,    mi, tex,
        x0, y1, z1,   t0, th, 0.0,    mi, tex,
        x1, y1, z1,   tw, th, 0.0,    mi, tex,
    ];
    vb.extend_from_slice(&cube_geometry);
    *element_count += 12;
}
