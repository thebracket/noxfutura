use crate::engine::VertexBuffer;
use crate::planet::{
    planet_idx, Planet, REGION_HEIGHT, REGION_WIDTH,
    noise_helper::{lat_to_y, lon_to_x}
};
use parking_lot::Mutex;
mod planet_render;
use planet_render::{add_point, altitude_to_color, landblock_to_color, build_blank_planet};

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
        build_blank_planet(&mut wgpr.vertex_buffer);
        wgpr
    }

    pub fn planet_with_altitude(&mut self, planet: Planet) {
        self.vertex_buffer.clear();
        const LAT_STEP: f32 = 1.0;
        const LON_STEP: f32 = 1.0;

        let mut lat = -90.0;
        let mut lon;
        const ALTITUDE_DIVISOR: f32 = 8192.0;

        while lat < 90.0-LAT_STEP {
            lon = -180.0;
            while lon < 180.0-LON_STEP {
                add_point(
                    &mut self.vertex_buffer,
                    lat,
                    lon,
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].height as f32
                        / ALTITUDE_DIVISOR,
                    &altitude_to_color(
                        planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].height,
                    ),
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat + LAT_STEP,
                    lon,
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat + LAT_STEP))].height
                        as f32
                        / ALTITUDE_DIVISOR,
                    &altitude_to_color(
                        planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat + LAT_STEP))]
                            .height,
                    ),
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat + LAT_STEP,
                    lon + LON_STEP,
                    planet.landblocks
                        [planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))]
                    .height as f32
                        / ALTITUDE_DIVISOR,
                    &altitude_to_color(
                        planet.landblocks
                            [planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))]
                        .height,
                    ),
                );

                add_point(
                    &mut self.vertex_buffer,
                    lat,
                    lon + LON_STEP,
                    planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat))].height
                        as f32
                        / ALTITUDE_DIVISOR,
                    &altitude_to_color(
                        planet.landblocks
                            [planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))]
                        .height,
                    ),
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat,
                    lon,
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].height as f32
                        / ALTITUDE_DIVISOR,
                    &altitude_to_color(
                        planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].height,
                    ),
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat + LAT_STEP,
                    lon + LON_STEP,
                    planet.landblocks
                        [planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))]
                    .height as f32
                        / ALTITUDE_DIVISOR,
                    &altitude_to_color(
                        planet.landblocks
                            [planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))]
                        .height,
                    ),
                );

                lon += LON_STEP;
            }
            lat += LAT_STEP;
        }

        self.needs_update = true;
    }

    pub fn planet_with_category(&mut self, planet: &Planet) {
        self.vertex_buffer.clear();
        const LAT_STEP: f32 = 1.0;
        const LON_STEP: f32 = 1.0;

        let mut lat = -90.0;
        let mut lon;
        const ALTITUDE_DIVISOR: f32 = 8192.0;

        while lat < 90.0 - LAT_STEP {
            lon = -180.0;
            while lon < 180.0 - LON_STEP {
                add_point(
                    &mut self.vertex_buffer,
                    lat,
                    lon,
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].height as f32
                        / ALTITUDE_DIVISOR,
                    &landblock_to_color(
                        &planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))],
                    ),
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat,
                    lon + LON_STEP,
                    planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat))].height
                        as f32
                        / ALTITUDE_DIVISOR,
                    &landblock_to_color(
                        &planet.landblocks
                            [planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))],
                    ),
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat + LAT_STEP,
                    lon,
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat + LAT_STEP))].height
                        as f32
                        / ALTITUDE_DIVISOR,
                    &landblock_to_color(
                        &planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat + LAT_STEP))],
                    ),
                );

                add_point(
                    &mut self.vertex_buffer,
                    lat,
                    lon + LON_STEP,
                    planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat))].height
                        as f32
                        / ALTITUDE_DIVISOR,
                    &landblock_to_color(
                        &planet.landblocks
                            [planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))],
                    ),
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat + LAT_STEP,
                    lon,
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat + LAT_STEP))].height
                        as f32
                        / ALTITUDE_DIVISOR,
                    &landblock_to_color(
                        &planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat + LAT_STEP))],
                    ),
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat + LAT_STEP,
                    lon + LON_STEP,
                    planet.landblocks
                        [planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))]
                    .height as f32
                        / ALTITUDE_DIVISOR,
                    &landblock_to_color(
                        &planet.landblocks
                            [planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))],
                    ),
                );

                lon += LON_STEP;
            }
            lat += LAT_STEP;
        }

        self.needs_update = true;
    }

    fn biome_to_color(&self, index: usize, planet: &Planet) -> [f32; 4] {
        let biome_index = planet.landblocks[index].biome_idx;
        if biome_index == std::usize::MAX {
            landblock_to_color(&planet.landblocks[index])
        } else {
            let biome_type_idx = planet.biomes[biome_index].biome_type;
            if biome_type_idx == std::usize::MAX {
                landblock_to_color(&planet.landblocks[index])
            } else {
                let lb_color = landblock_to_color(&planet.landblocks[index]);
                let base_color = crate::raws::RAWS.lock().biomes.areas[biome_type_idx]
                    .color
                    .clone();
                [
                    (base_color[0] + lb_color[0]) / 2.0,
                    (base_color[1] + lb_color[1]) / 2.0,
                    (base_color[2] + lb_color[2]) / 2.0,
                    1.0,
                ]
            }
        }
    }

    pub fn planet_with_biome(&mut self, planet: &Planet) {
        self.vertex_buffer.clear();
        const LAT_STEP: f32 = 1.0;
        const LON_STEP: f32 = 1.0;

        let mut lat = -90.0;
        let mut lon;
        const ALTITUDE_DIVISOR: f32 = 8192.0;

        while lat < 90.0 - LAT_STEP {
            lon = -180.0;
            while lon < 180.0 - LON_STEP {
                let bcolor = self.biome_to_color(planet_idx(lon_to_x(lon), lat_to_y(lat)), &planet);
                add_point(
                    &mut self.vertex_buffer,
                    lat,
                    lon,
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].height as f32
                        / ALTITUDE_DIVISOR,
                    &bcolor,
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat,
                    lon + LON_STEP,
                    planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat))].height
                        as f32
                        / ALTITUDE_DIVISOR,
                    &bcolor,
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat + LAT_STEP,
                    lon,
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat + LAT_STEP))].height
                        as f32
                        / ALTITUDE_DIVISOR,
                    &bcolor,
                );

                add_point(
                    &mut self.vertex_buffer,
                    lat,
                    lon + LON_STEP,
                    planet.landblocks[planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat))].height
                        as f32
                        / ALTITUDE_DIVISOR,
                    &bcolor,
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat + LAT_STEP,
                    lon,
                    planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat + LAT_STEP))].height
                        as f32
                        / ALTITUDE_DIVISOR,
                    &bcolor,
                );
                add_point(
                    &mut self.vertex_buffer,
                    lat + LAT_STEP,
                    lon + LON_STEP,
                    planet.landblocks
                        [planet_idx(lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP))]
                    .height as f32
                        / ALTITUDE_DIVISOR,
                    &bcolor,
                );

                lon += LON_STEP;
            }
            lat += LAT_STEP;
        }

        self.needs_update = true;
    }

    fn hm_to_z(&self, height: u8) -> f32 {
        height as f32 / 255.0
    }

    pub fn region_heightmap(&mut self, hm: &[u8], water_level: u8, water: &[u8]) {
        self.vertex_buffer.clear();
        const SCALE: f32 = 512.0;
        const HRW: f32 = (REGION_WIDTH as f32 / 2.0) / SCALE;
        const HRH: f32 = (REGION_HEIGHT as f32 / 2.0) / SCALE;
        let min_height = hm.iter().min().unwrap();
        let max_height = hm.iter().max().unwrap();
        println!("{},{}", min_height, max_height);
        let altitude_range = max_height - min_height;

        for idx in 0..hm.len() - (REGION_WIDTH + 1) as usize {
            let height = hm[idx];
            let mag = (height - min_height) as f32 / altitude_range as f32;
            //let mag = *height as f32 / 255.0;
            let x = idx % REGION_WIDTH as usize;
            let y = idx / REGION_WIDTH as usize;

            let z00 = self.hm_to_z(height);
            let z10 = self.hm_to_z(hm[idx + 1]);
            let z01 = self.hm_to_z(hm[idx + REGION_WIDTH as usize]);
            let z11 = self.hm_to_z(hm[idx + 1 + REGION_WIDTH as usize]);

            let (r, g, b) = if height < water_level || water[idx] > height {
                (0.0, 0.0, 1.0)
            } else {
                (0.0, mag, 0.0)
            };

            let x1 = (x as f32 / SCALE) - HRW;
            let x2 = ((x + 1) as f32 / SCALE) - HRW;
            let y1 = (y as f32 / SCALE) - HRH;
            let y2 = ((y + 1) as f32 / SCALE) - HRH;

            self.vertex_buffer.add3(x1, y2, z01);
            self.vertex_buffer.add4(r, g, b, 1.0);
            self.vertex_buffer.add3(x1, y1, z00);
            self.vertex_buffer.add4(r, g, b, 1.0);
            self.vertex_buffer.add3(x2, y1, z10);
            self.vertex_buffer.add4(r, g, b, 1.0);
            self.vertex_buffer.add3(x1, y2, z01);
            self.vertex_buffer.add4(r, g, b, 1.0);
            self.vertex_buffer.add3(x2, y1, z10);
            self.vertex_buffer.add4(r, g, b, 1.0);
            self.vertex_buffer.add3(x2, y2, z11);
            self.vertex_buffer.add4(r, g, b, 1.0);
        }
        self.needs_update = true;
    }

    pub fn region_display_primitives(&mut self, primitives: Vec<crate::region::Primitive>) {
        self.vertex_buffer.clear();
        primitives.iter().for_each(|p| {
            match *p {
                crate::region::Primitive::Cube{x, y, z, w, h, d} => {
                    //println!("{},{},{} .. {},{},{}", x, y, z, w, h, d);
                    //self.add_cube(x, y, z, w, h, d);
                    crate::utils::add_cube_geometry(&mut self.vertex_buffer, x as f32, y as f32, z as f32, w as f32, h as f32, d as f32);
                }
            }
        });
        self.needs_update = true;
    }
}
