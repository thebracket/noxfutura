use crate::engine::VertexBuffer;
use crate::planet::{planet_idx, Planet, WORLD_HEIGHT, WORLD_WIDTH};
use super::noise_helper::{lat_to_y, lon_to_x};
use parking_lot::Mutex;

lazy_static! {
    pub static ref WORLDGEN_RENDER: Mutex<WorldGenPlanetRender> =
        Mutex::new(WorldGenPlanetRender::new());
}

pub struct WorldGenPlanetRender {
    pub vertex_buffer: VertexBuffer<f32>,
    pub needs_update: bool,
}

impl WorldGenPlanetRender {
    fn new() -> Self {
        let mut wgpr = Self {
            vertex_buffer: VertexBuffer::new(&[3, 4]),
            needs_update: false,
        };
        wgpr.build_blank_planet();
        wgpr
    }

    fn sphere_vertex(&self, altitude: f32, lat: f32, lon: f32) -> (f32, f32, f32) {
        (
            altitude * f32::cos(lat) * f32::cos(lon),
            altitude * f32::cos(lat) * f32::sin(lon),
            altitude * f32::sin(lat),
        )
    }

    fn add_point(&mut self, lat: f32, lon: f32, altitude: f32, color: &[f32; 4]) {
        let latr = lat * 0.0174533;
        let lonr = lon * 0.0174533;
        let sphere_coords = self.sphere_vertex(0.5 + altitude, latr, lonr);
        self.vertex_buffer
            .add3(sphere_coords.0, sphere_coords.1, sphere_coords.2);

        self.vertex_buffer.add_slice(color);
    }

    pub fn build_blank_planet(&mut self) {
        const LAT_STEP: f32 = WORLD_HEIGHT as f32 / 180.0;
        const LON_STEP: f32 = WORLD_WIDTH as f32 / 360.0;

        let mut lat = -90.0;
        let mut lon;
        let blue = [0.0, 0.0, 1.0, 1.0];

        while lat < 90.0 {
            lon = -180.0;
            while lon < 180.0 {
                self.add_point(lat, lon, 0.0, &blue);
                self.add_point(lat + LAT_STEP, lon, 0.0, &blue);
                self.add_point(lat + LAT_STEP, lon + LON_STEP, 0.0, &blue);

                lon += LON_STEP;
            }
            lat += LAT_STEP;
        }
    }

    fn altitude_to_color(&self, altitude : u8) -> [f32; 4] {
        let mag = altitude as f32 / 255.0;
        [mag, mag, mag, 1.0]
    }

    pub fn planet_with_altitude(&mut self, planet: Planet) {
        self.vertex_buffer.clear();
        const LAT_STEP: f32 = 1.0;
        const LON_STEP: f32 = 1.0;

        let mut lat = -90.0;
        let mut lon;
        const ALTITUDE_DIVISOR: f32 = 8192.0;

        while lat < 90.0 {
            lon = -180.0;
            while lon < 180.0 {
                self.add_point(
                    lat, 
                    lon, 
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].height as f32 / ALTITUDE_DIVISOR, 
                    &self.altitude_to_color(planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].height),
                );
                self.add_point(
                    lat + LAT_STEP,
                    lon,
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat + LAT_STEP))].height as f32 / ALTITUDE_DIVISOR,
                    &self.altitude_to_color(planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat + LAT_STEP))].height),
                );
                self.add_point(
                    lat + LAT_STEP,
                    lon + LON_STEP,
                    planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))].height as f32 / ALTITUDE_DIVISOR,
                    &self.altitude_to_color(planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))].height),
                );

                self.add_point(
                    lat + LAT_STEP,
                    lon + LON_STEP,
                    planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))].height as f32 / ALTITUDE_DIVISOR,
                    &self.altitude_to_color(planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))].height),
                );
                self.add_point(
                    lat,
                    lon + LON_STEP,
                    planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat))].height as f32 / ALTITUDE_DIVISOR,
                    &self.altitude_to_color(planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))].height),
                );
                self.add_point(
                    lat, 
                    lon, 
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].height as f32 / ALTITUDE_DIVISOR, 
                    &self.altitude_to_color(planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].height),
                );

                lon += LON_STEP;
            }
            lat += LAT_STEP;
        }

        self.needs_update = true;
    }
}
