use nox_raws::RAWS;

pub fn add_floor_geometry(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    material_index: (usize, bool),
) {
    let rlock = RAWS.read();
    let mi = *rlock.matmap.get(material_index.0) as f32;
    let tex = if material_index.1 {
        rlock.materials.material_list.as_ref().unwrap()[material_index.0].floor_constructed
    } else {
        rlock.materials.material_list.as_ref().unwrap()[material_index.0].floor
    };

    let x0 = x;
    let x1 = x0 + w;
    let y0 = z - 0.0; // was 0.1
    let y1 = y0 + 0.0; // was 0.11
    let z0 = y;
    let z1 = z0 + h;

    let t0 = 0.0f32;
    let tw = w;
    let th = h;

    #[rustfmt::skip]
    let cube_geometry = [
        x1, y1, z1, tw, th,  0.0,  mi, tex,
        x1, y1, z0, tw, t0,  0.0,  mi, tex,
        x0, y1, z0, t0, t0,  0.0,  mi, tex,
        x0, y1, z0, t0, t0,  0.0,  mi, tex,
        x0, y1, z1, t0, th,  0.0,  mi, tex,
        x1, y1, z1, tw, th,  0.0,  mi, tex,
    ];
    vb.extend_from_slice(&cube_geometry);
    *element_count += 2;
}
