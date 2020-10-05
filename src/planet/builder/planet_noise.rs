use super::noise_helper::*;
use super::{set_worldgen_status, Block, BlockType, PLANET_BUILD};
use crate::planet::{planet_idx, sphere_vertex};
use crate::spatial::{REGION_HEIGHT, REGION_WIDTH, WORLD_HEIGHT, WORLD_TILES_COUNT, WORLD_WIDTH};
use bracket_geometry::prelude::Degrees;
use bracket_noise::prelude::*;

pub(crate) fn zero_fill() {
    set_worldgen_status("Building initial ball of mud");
    let blocks: Vec<Block> = vec![Block::blank(); WORLD_TILES_COUNT as usize];
    PLANET_BUILD.lock().planet.landblocks = blocks;
    PLANET_BUILD.lock().planet.migrant_counter = 0;
    PLANET_BUILD.lock().planet.remaining_settlers = 0;
}

pub(crate) fn planetary_noise() {
    set_worldgen_status("Dividing the heavens from the earth");
    let perlin_seed = PLANET_BUILD.lock().planet.perlin_seed;
    let mut noise = FastNoise::seeded(perlin_seed);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(5);
    noise.set_fractal_gain(0.5);
    noise.set_fractal_lacunarity(3.0);
    noise.set_frequency(0.01);

    let max_temperature = 56.7;
    let min_temperature = -55.2;
    let temperature_range = max_temperature - min_temperature;
    let half_planet_height = WORLD_HEIGHT as f32 / 2.0;
    const REGION_FRACTION_TO_CONSIDER: usize = 64;

    for y in 0..WORLD_HEIGHT {
        let distance_from_equator = i32::abs((WORLD_HEIGHT as i32 / 2i32) - y as i32);
        let temp_range_percent = 1.0 - distance_from_equator as f32 / half_planet_height;
        let base_temp_by_latitude = (temp_range_percent * temperature_range) + min_temperature;
        for x in 0..WORLD_WIDTH {
            let mut total_height = 0u32;

            let mut max = 0;
            let mut min = std::u8::MAX;
            let mut n_tiles = 0;
            for y1 in 0..REGION_HEIGHT / REGION_FRACTION_TO_CONSIDER {
                for x1 in 0..REGION_WIDTH / REGION_FRACTION_TO_CONSIDER {
                    let lat = noise_lat(y, y1 * REGION_FRACTION_TO_CONSIDER);
                    let lon = noise_lon(x, x1 * REGION_FRACTION_TO_CONSIDER);
                    let sphere_coords = sphere_vertex(100.0, Degrees::new(lat), Degrees::new(lon));
                    let nh = noise.get_noise3d(sphere_coords.0, sphere_coords.1, sphere_coords.2);
                    /*let nh = noise.get_noise(
                        noise_x(x, x1 * REGION_FRACTION_TO_CONSIDER),
                        noise_y(y, y1 * REGION_FRACTION_TO_CONSIDER),
                    );*/
                    let n = noise_to_planet_height(nh);
                    if n < min {
                        min = n
                    }
                    if n > max {
                        max = n
                    }
                    total_height += n as u32;
                    n_tiles += 1;
                }
            }

            let pidx = planet_idx(x, y);
            let mut planet = PLANET_BUILD.lock();
            planet.planet.landblocks[pidx].height = (total_height / n_tiles as u32) as u8;
            //println!("{}", planet.planet.landblocks[pidx].height);
            planet.planet.landblocks[pidx].btype = BlockType::None;
            planet.planet.landblocks[pidx].variance = max - min;
            let altitude_deduction = (planet.planet.landblocks[pidx].height as f32
                - planet.planet.water_height as f32)
                / 10.0;
            planet.planet.landblocks[pidx].temperature =
                (base_temp_by_latitude - altitude_deduction) as i8;
            if planet.planet.landblocks[pidx].temperature < -55 {
                planet.planet.landblocks[pidx].temperature = -55
            }
            if planet.planet.landblocks[pidx].temperature > 55 {
                planet.planet.landblocks[pidx].temperature = 55
            }
            std::mem::drop(planet);
        }

        let percent = y as f32 / WORLD_HEIGHT as f32;
        set_worldgen_status(format!(
            "Dividing heavens from the earth: {}%",
            (percent * 100.0) as u8
        ));

        if y % 10 == 0 {
            let planet_copy = PLANET_BUILD.lock().planet.clone();
            use crate::planet::WORLDGEN_RENDER;
            WORLDGEN_RENDER.lock().planet_with_altitude(planet_copy);
        }
    }
}
