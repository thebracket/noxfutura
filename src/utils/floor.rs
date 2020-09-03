use crate::raws::MappedTexture;

pub fn add_floor_geometry(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    material_index: MappedTexture,
) {
    let tint = material_index.tint;
    let mi = material_index.texture as f32;
    let x0 = x;
    let x1 = x0 + w;
    let y0 = z - 0.1;
    let y1 = y0 + 0.11;
    let z0 = y;
    let z1 = z0 + h;

    let t0 = 0.0f32;
    let tw = w;
    let th = h;

    #[rustfmt::skip]
    let cube_geometry = [
        x1, y1, z1,   0.0,     tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y1, z0,   0.0,     tw, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z0,   0.0,     t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z0,   0.0,     t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z1,   0.0,     t0, th, mi, tint.0, tint.1, tint. 2,
        x1, y1, z1,   0.0,     tw, th, mi, tint.0, tint.1, tint. 2,
    ];
    vb.extend_from_slice(&cube_geometry);
    *element_count += 2;
}
