use crate::{
    noise_helper::{lat_to_y, lon_to_x},
    planet_idx, sphere_vertex, Block, BlockType,
};
use bracket_geometry::prelude::Degrees;
use cgmath::Vector3;
use nox_spatial::{WORLD_HEIGHT, WORLD_WIDTH};
use nox_wgpu_utils::VertexBuffer;

pub type LatLonIdx = (f32, f32, usize);
pub type LatLonQuad = [LatLonIdx; 6];

pub fn planet_quad_coords(lat: f32, lon: f32, lat_step: f32, lon_step: f32) -> LatLonQuad {
    [
        (lat, lon, planet_idx(lon_to_x(lon), lat_to_y(lat))),
        (
            lat,
            lon + lon_step,
            planet_idx(lon_to_x(lon + lon_step), lat_to_y(lat)),
        ),
        (
            lat + lat_step,
            lon,
            planet_idx(lon_to_x(lon), lat_to_y(lat + lat_step)),
        ),
        (
            lat,
            lon + lon_step,
            planet_idx(lon_to_x(lon + lon_step), lat_to_y(lat)),
        ),
        (
            lat + lat_step,
            lon,
            planet_idx(lon_to_x(lon), lat_to_y(lat + lat_step)),
        ),
        (
            lat + lat_step,
            lon + lon_step,
            planet_idx(lon_to_x(lon + lon_step), lat_to_y(lat + lat_step)),
        ),
    ]
}

pub fn all_planet_points<F: FnMut(&LatLonIdx)>(mut point_maker: F) {
    const LAT_STEP: f32 = 1.0;
    const LON_STEP: f32 = 1.0;

    let mut lat = -90.0;
    let mut lon;

    while lat < 90.0 - LAT_STEP {
        lon = -180.0;
        while lon < 180.0 - LON_STEP {
            planet_quad_coords(lat, lon, LAT_STEP, LON_STEP)
                .iter()
                .for_each(|l| point_maker(l));
            lon += LON_STEP;
        }
        lat += LAT_STEP;
    }
}

pub fn add_point(
    vertex_buffer: &mut VertexBuffer<f32>,
    lat: f32,
    lon: f32,
    altitude: f32,
    color: &[f32; 4],
) {
    use cgmath::InnerSpace;
    let sphere_coords = sphere_vertex(0.75 + altitude, Degrees::new(lat), Degrees::new(lon));
    vertex_buffer.add3(sphere_coords.0, sphere_coords.1, sphere_coords.2);
    let mut normals = Vector3::from(sphere_coords);
    normals = normals.normalize();
    vertex_buffer.add3(normals.x, normals.y, normals.z);
    vertex_buffer.add4(color[0], color[1], color[2], color[3]);
}

pub fn altitude_to_color(altitude: u8) -> [f32; 4] {
    let mag = altitude as f32 / 255.0;
    [mag, mag, mag, 1.0]
}

pub fn landblock_to_color(lb: &Block) -> [f32; 4] {
    let mut color = match lb.btype {
        BlockType::Coastal => [1.0, 1.0, 0.0, 1.0],
        BlockType::Highlands => [0.7, 0.7, 0.7, 1.0],
        BlockType::Hills => [0.5, 0.5, 0.5, 1.0],
        BlockType::Marsh => [0.2, 1.0, 0.4, 1.0],
        BlockType::Mountains => [0.9, 0.9, 0.9, 1.0],
        BlockType::Plains => [0.0, 1.0, 0.0, 1.0],
        BlockType::Plateau => [0.7, 0.9, 0.7, 1.0],
        BlockType::SaltMarsh => [0.2, 1.0, 0.4, 1.0],
        BlockType::Water => [0.0, 0.0, 1.0, 1.0],
        _ => altitude_to_color(lb.height),
    };
    let mag = lb.height as f32 / 255.0;
    color.iter_mut().for_each(|c| *c *= mag);
    color
}

pub fn build_blank_planet(vertex_buffer: &mut VertexBuffer<f32>) {
    const LAT_STEP: f32 = WORLD_HEIGHT as f32 / 180.0;
    const LON_STEP: f32 = WORLD_WIDTH as f32 / 360.0;

    let mut lat = -90.0;
    let mut lon;
    let blue = [0.0, 0.0, 1.0, 1.0];

    while lat < 90.0 {
        lon = -180.0;
        while lon < 180.0 {
            add_point(vertex_buffer, lat, lon, 0.0, &blue);
            add_point(vertex_buffer, lat + LAT_STEP, lon, 0.0, &blue);
            add_point(vertex_buffer, lat + LAT_STEP, lon + LON_STEP, 0.0, &blue);

            lon += LON_STEP;
        }
        lat += LAT_STEP;
    }
}
