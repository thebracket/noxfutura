use crate::planet::RampDirection;
use nox_raws::MappedTexture;

pub fn add_ramp_geometry(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    direction: RampDirection,
    x: f32,
    y: f32,
    z: f32,
    material_index: MappedTexture,
) {
    let tint = material_index.tint;
    match direction {
        RampDirection::NorthSouth => north_south(
            vb,
            element_count,
            x,
            y,
            z,
            material_index.texture as f32 / 255.0,
            tint,
        ),
        RampDirection::SouthNorth => south_north(
            vb,
            element_count,
            x,
            y,
            z,
            material_index.texture as f32 / 255.0,
            tint,
        ),
        RampDirection::EastWest => east_west(
            vb,
            element_count,
            x,
            y,
            z,
            material_index.texture as f32 / 255.0,
            tint,
        ),
        RampDirection::WestEast => west_east(
            vb,
            element_count,
            x,
            y,
            z,
            material_index.texture as f32 / 255.0,
            tint,
        ), //_ => {}
    }
}

fn north_south(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32,
    mi: f32,
    tint: (f32, f32, f32),
) {
    let w = 1.0;
    let h = 1.0;
    let d = 1.0;

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
        x0, y0, z0,    1.0,  t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y1, z0,    1.0,  tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z0,    1.0,  tw, t0, mi, tint.0, tint.1, tint. 2,
        x1, y1, z0,    1.0,  tw, th, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,    1.0,  t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z0,    1.0,  t0, th, mi, tint.0, tint.1, tint. 2,

        // Left side
        x0, y0, z0,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,
        x0, y1, z0,   5.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,


        // Right side
        x1, y0, z0,    3.0,   tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y1, z0,    3.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,    3.0,   t0, t0, mi, tint.0, tint.1, tint. 2,

        // Base - unchanged
        x0, y0, z0,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z0,   5.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,   5.0,   t0, th, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,

        // Top - needs to slope
        x1, y0, z1,   0.0,    tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y1, z0,   0.0,    tw, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z0,   0.0,    t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z0,   0.0,    t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,   0.0,    t0, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,   0.0,    tw, th, mi, tint.0, tint.1, tint. 2,
    ];
    vb.extend_from_slice(&cube_geometry);
    *element_count += 8;
}

// still needs work
fn south_north(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32,
    mi: f32,
    tint: (f32, f32, f32),
) {
    let w = 1.0;
    let h = 1.0;
    let d = 1.0;

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
        /*
        x0, y0, z0,    1.0,  t0, t0,
        x1, y1, z0,    1.0,  tw, th,
        x1, y0, z0,    1.0,  tw, t0,
        x1, y1, z0,    1.0,  tw, th,
        x0, y0, z0,    1.0,  t0, t0,
        x0, y1, z0,    1.0,  t0, th,*/

        x0, y0, z1,    2.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,    2.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x1, y1, z1,    2.0,   tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y1, z1,    2.0,   tw, th, mi, tint.0, tint.1, tint. 2,
        x0, y1, z1,    2.0,   t0, th, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,    2.0,   t0, t0, mi, tint.0, tint.1, tint. 2,

        /*
        x0, y1, z1,   5.0,   tw, th,
        x0, y1, z0,   5.0,   tw, t0,
        x0, y0, z0,   5.0,   t0, t0,
        x0, y0, z0,   5.0,   t0, t0,
        x0, y0, z1,   5.0,   t0, th,
        x0, y1, z1,   5.0,   tw, th,
        */
        x0, y0, z0,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,   5.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z1,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,

        /*
        x1, y1, z1,    3.0,  tw, th,
        x1, y0, z0,    3.0,  t0, t0,
        x1, y1, z0,    3.0,  tw, t0,
        x1, y0, z0,    3.0,  t0, t0,
        x1, y1, z1,    3.0,  tw, th,
        x1, y0, z1,    3.0,  t0, th,
        */
        x1, y0, z0,    3.0,  tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y1, z1,    3.0,  t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,    3.0,  tw, t0, mi, tint.0, tint.1, tint. 2,

        // Base
        x0, y0, z0,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z0,   5.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,   5.0,   t0, th, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,

        // Top
        x1, y1, z1,   0.0,    tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z0,   0.0,    tw, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,   0.0,    t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,   0.0,    t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z1,   0.0,    t0, th, mi, tint.0, tint.1, tint. 2,
        x1, y1, z1,   0.0,    tw, th, mi, tint.0, tint.1, tint. 2,
    ];
    vb.extend_from_slice(&cube_geometry);
    *element_count += 8;
}

fn east_west(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32,
    mi: f32,
    tint: (f32, f32, f32),
) {
    let w = 1.0;
    let h = 1.0;
    let d = 1.0;

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
        /*
        x0, y0, z0,    1.0,  t0, t0,
        x1, y1, z0,    1.0,  tw, th,
        x1, y0, z0,    1.0,  tw, t0,
        x1, y1, z0,    1.0,  tw, th,
        x0, y0, z0,    1.0,  t0, t0,
        x0, y1, z0,    1.0,  t0, th,*/
        x1, y0, z0,    1.0,  t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,    1.0,  tw, th, mi, tint.0, tint.1, tint. 2,
        x0, y1, z0,    1.0,  tw, t0, mi, tint.0, tint.1, tint. 2,

        /*
        x0, y0, z1,    2.0,   t0, t0,
        x1, y0, z1,    2.0,   tw, t0,
        x1, y1, z1,    2.0,   tw, th,
        x1, y1, z1,    2.0,   tw, th,
        x0, y1, z1,    2.0,   t0, th,
        x0, y0, z1,    2.0,   t0, t0,
        */
        x1, y0, z1,    2.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,    2.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z1,    2.0,   tw, th, mi, tint.0, tint.1, tint. 2,

        x0, y1, z1,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,
        x0, y1, z0,   5.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,   5.0,   t0, th, mi, tint.0, tint.1, tint. 2,
        x0, y1, z1,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,

        /*x1, y1, z1,    3.0,  tw, th,
        x1, y0, z0,    3.0,  t0, t0,
        x1, y1, z0,    3.0,  tw, t0,
        x1, y0, z0,    3.0,  t0, t0,
        x1, y1, z1,    3.0,  tw, th,
        x1, y0, z1,    3.0,  t0, th,*/

        // Base
        x0, y0, z0,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z0,   5.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,   5.0,   t0, th, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,

        // Top - slope me
        x1, y0, z1,   0.0,    tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z0,   0.0,    tw, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z0,   0.0,    t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z0,   0.0,    t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y1, z1,   0.0,    t0, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,   0.0,    tw, th, mi, tint.0, tint.1, tint. 2,
    ];
    vb.extend_from_slice(&cube_geometry);
    *element_count += 8;
}

fn west_east(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32,
    mi: f32,
    tint: (f32, f32, f32),
) {
    let w = 1.0;
    let h = 1.0;
    let d = 1.0;

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
        /*
        x0, y0, z0,    1.0,  t0, t0,
        x1, y1, z0,    1.0,  tw, th,
        x1, y0, z0,    1.0,  tw, t0,
        x1, y1, z0,    1.0,  tw, th,
        x0, y0, z0,    1.0,  t0, t0,
        x0, y1, z0,    1.0,  t0, th,
        */
        x0, y0, z0,    1.0,  t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y1, z0,    1.0,  tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z0,    1.0,  tw, t0, mi, tint.0, tint.1, tint. 2,

        /*
        x0, y0, z1,    2.0,   t0, t0,
        x1, y0, z1,    2.0,   tw, t0,
        x1, y1, z1,    2.0,   tw, th,
        x1, y1, z1,    2.0,   tw, th,
        x0, y1, z1,    2.0,   t0, th,
        x0, y0, z1,    2.0,   t0, t0,
        */
        x0, y0, z1,    2.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,    2.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x1, y1, z1,    2.0,   tw, th, mi, tint.0, tint.1, tint. 2,

        x1, y1, z1,    3.0,  tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z0,    3.0,  t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y1, z0,    3.0,  tw, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z0,    3.0,  t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y1, z1,    3.0,  tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,    3.0,  t0, th, mi, tint.0, tint.1, tint. 2,

        // Base - unchanged
        x0, y0, z0,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y0, z0,   5.0,   tw, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x1, y0, z1,   5.0,   t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,   5.0,   t0, th, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,   5.0,   tw, th, mi, tint.0, tint.1, tint. 2,

        // Top - needs to slope
        x1, y1, z1,   0.0,    tw, th, mi, tint.0, tint.1, tint. 2,
        x1, y1, z0,   0.0,    tw, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,   0.0,    t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z0,   0.0,    t0, t0, mi, tint.0, tint.1, tint. 2,
        x0, y0, z1,   0.0,    t0, th, mi, tint.0, tint.1, tint. 2,
        x1, y1, z1,   0.0,    tw, th, mi, tint.0, tint.1, tint. 2,
    ];
    vb.extend_from_slice(&cube_geometry);
    *element_count += 8;
}
