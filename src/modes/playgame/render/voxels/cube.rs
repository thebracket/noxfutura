const MODEL_SCALE: f32 = 1.0 / 32.0;
use crate::modes::playgame::Palette;

pub fn add_cube_geometry(
    vb: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    d: f32,
    material_index: u8,
    palette: &Palette,
) {
    let x0 = x * MODEL_SCALE;
    let x1 = x0 + (w * MODEL_SCALE);
    let y0 = z * MODEL_SCALE;
    let y1 = y0 + (d * MODEL_SCALE);
    let z0 = y * MODEL_SCALE;
    let z1 = z0 + (h * MODEL_SCALE);

    let color_bytes = dot_vox::DEFAULT_PALETTE[material_index as usize];

    let rr: u8 = ((color_bytes & 0x00ff0000) >> 16) as u8;
    let rg: u8 = ((color_bytes & 0x0000ff00) >> 8) as u8;
    let rb: u8 = (color_bytes & 0x000000ff) as u8;

    let b = rr as f32 / 255.0;
    let g = rg as f32 / 255.0;
    let r = rb as f32 / 255.0;

    let palette_entry = palette.find_palette(r, g, b) as f32;

    #[rustfmt::skip]
    let cube_geometry = [
        x0, y0, z0,    1.0,  palette_entry,
        x1, y1, z0,    1.0,  palette_entry,
        x1, y0, z0,    1.0,  palette_entry,
        x1, y1, z0,    1.0,  palette_entry,
        x0, y0, z0,    1.0,  palette_entry,
        x0, y1, z0,    1.0,  palette_entry,

        x0, y0, z1,    2.0,   palette_entry,
        x1, y0, z1,    2.0,   palette_entry,
        x1, y1, z1,    2.0,   palette_entry,
        x1, y1, z1,    2.0,   palette_entry,
        x0, y1, z1,    2.0,   palette_entry,
        x0, y0, z1,    2.0,   palette_entry,

        x0, y1, z1,    3.0,   palette_entry,
        x0, y1, z0,    3.0,   palette_entry,
        x0, y0, z0,    3.0,   palette_entry,
        x0, y0, z0,    3.0,   palette_entry,
        x0, y0, z1,    3.0,   palette_entry,
        x0, y1, z1,    3.0,   palette_entry,

        x1, y1, z1,    4.0,  palette_entry,
        x1, y0, z0,    4.0,  palette_entry,
        x1, y1, z0,    4.0,  palette_entry,
        x1, y0, z0,    4.0,  palette_entry,
        x1, y1, z1,    4.0,  palette_entry,
        x1, y0, z1,    4.0,  palette_entry,

        x0, y0, z0,   5.0,   palette_entry,
        x1, y0, z0,   5.0,   palette_entry,
        x1, y0, z1,   5.0,   palette_entry,
        x1, y0, z1,   5.0,   palette_entry,
        x0, y0, z1,   5.0,   palette_entry,
        x0, y0, z0,   5.0,   palette_entry,

        x1, y1, z1,   0.0,    palette_entry,
        x1, y1, z0,   0.0,    palette_entry,
        x0, y1, z0,   0.0,    palette_entry,
        x0, y1, z0,   0.0,    palette_entry,
        x0, y1, z1,   0.0,    palette_entry,
        x1, y1, z1,   0.0,    palette_entry,
    ];
    vb.extend_from_slice(&cube_geometry);
}
