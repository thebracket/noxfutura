use crate::planet::RampDirection;

pub fn add_ramp_geometry(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    direction: RampDirection,
    x: f32,
    y: f32,
    z: f32
) {
    match direction {
        RampDirection::NorthSouth => north_south(vb, element_count, x, y, z),
        RampDirection::SouthNorth => south_north(vb, element_count, x, y, z),
        RampDirection::EastWest => east_west(vb, element_count, x, y, z),
        RampDirection::WestEast => west_east(vb, element_count, x, y, z),
        _ => {}
    }
}

fn north_south(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32
) {
    let w = 1.0;
    let h = 1.0;
    let d = 0.5;

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
        x0, y0, z0,    0.0, 0.0, -1.0,  t0, t0,
        x1, y1, z0,    0.0, 0.0, -1.0,  tw, th,
        x1, y0, z0,    0.0, 0.0, -1.0,  tw, t0,
        x1, y1, z0,    0.0, 0.0, -1.0,  tw, th,
        x0, y0, z0,    0.0, 0.0, -1.0,  t0, t0,
        x0, y1, z0,    0.0, 0.0, -1.0,  t0, th,

        x0, y0, z1,    0.0, 0.0, 1.0,   t0, t0,
        x1, y0, z1,    0.0, 0.0, 1.0,   tw, t0,
        x1, y1, z1,    0.0, 0.0, 1.0,   tw, th,
        x1, y1, z1,    0.0, 0.0, 1.0,   tw, th,
        x0, y1, z1,    0.0, 0.0, 1.0,   t0, th,
        x0, y0, z1,    0.0, 0.0, 1.0,   t0, t0,

        x0, y1, z1,    1.0, 0.0, 0.0,   tw, th,
        x0, y1, z0,    1.0, 0.0, 0.0,   tw, t0,
        x0, y0, z0,    1.0, 0.0, 0.0,   t0, t0,
        x0, y0, z0,    1.0, 0.0, 0.0,   t0, t0,
        x0, y0, z1,    1.0, 0.0, 0.0,   t0, th,
        x0, y1, z1,    1.0, 0.0, 0.0,   tw, th,

        x1, y1, z1,    -1.0, 0.0, 0.0,  tw, th,
        x1, y0, z0,    -1.0, 0.0, 0.0,  t0, t0,
        x1, y1, z0,    -1.0, 0.0, 0.0,  tw, t0,
        x1, y0, z0,    -1.0, 0.0, 0.0,  t0, t0,
        x1, y1, z1,    -1.0, 0.0, 0.0,  tw, th,
        x1, y0, z1,    -1.0, 0.0, 0.0,  t0, th,

        x0, y0, z0,   0.0, -1.0, 0.0,   tw, th,
        x1, y0, z0,   0.0, -1.0, 0.0,   tw, t0,
        x1, y0, z1,   0.0, -1.0, 0.0,   t0, t0,
        x1, y0, z1,   0.0, -1.0, 0.0,   t0, t0,
        x0, y0, z1,   0.0, -1.0, 0.0,   t0, th,
        x0, y0, z0,   0.0, -1.0, 0.0,   tw, th,

        x1, y1, z1,   0.0, 1.0, 0.0,    tw, th,
        x1, y1, z0,   0.0, 1.0, 0.0,    tw, t0,
        x0, y1, z0,   0.0, 1.0, 0.0,    t0, t0,
        x0, y1, z0,   0.0, 1.0, 0.0,    t0, t0,
        x0, y1, z1,   0.0, 1.0, 0.0,    t0, th,
        x1, y1, z1,   0.0, 1.0, 0.0,    tw, th,
    ];
    vb.extend_from_slice(&cube_geometry);
    *element_count += 12;
}

fn south_north(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32
) {

}

fn east_west(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32
) {

}

fn west_east(
    vb: &mut Vec<f32>,
    element_count: &mut u32,
    x: f32,
    y: f32,
    z: f32
) {

}