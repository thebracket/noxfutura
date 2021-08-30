use super::{
    atmospheric_pressure_by_elevation, average_precipitation_mm_by_latitude,
    average_temperature_by_latitude, planet_3d::sphere_vertex, temperature_decrease_by_altitude,
    Planet, PlanetMesh, PLANET_GEN,
};
use crate::geometry::Degrees;
use crate::simulation::{
    noise_lat, noise_lon, planet_idx, REGION_HEIGHT, REGION_WIDTH, WORLD_HEIGHT, WORLD_WIDTH,
};
use bracket_noise::prelude::*;

fn noise_to_planet_height(n: f32) -> u8 {
    ((n + 1.0) * 150.0) as u8
}

pub fn planetary_noise(planet: &mut Planet) {
    const SAMPLE_DIVISOR: usize = 24;
    const X_SAMPLES: usize = REGION_WIDTH as usize / SAMPLE_DIVISOR;
    const Y_SAMPLES: usize = REGION_HEIGHT as usize / SAMPLE_DIVISOR;

    let mut noise = FastNoise::seeded(planet.noise_seed);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(5);
    noise.set_fractal_gain(0.5);
    noise.set_fractal_lacunarity(2.0);
    noise.set_frequency(0.01);

    for y in 0..WORLD_HEIGHT {
        let lat = Degrees::new(noise_lat(y, 0));
        let base_temperature_c = average_temperature_by_latitude(lat);
        let rainfall_mm = average_precipitation_mm_by_latitude(lat) / 3.0;

        for x in 0..WORLD_WIDTH {
            let mut total_height = 0u32;
            let mut tile_count = 0u32;
            let mut max = 0;
            let mut min = std::u8::MAX;
            let mut max_noise = 0.0;
            for y1 in 0..Y_SAMPLES {
                let lat = noise_lat(y, y1 * SAMPLE_DIVISOR);
                for x1 in 0..X_SAMPLES {
                    let lon = noise_lon(x, x1 * SAMPLE_DIVISOR);
                    let sphere_coords = sphere_vertex(100.0, Degrees::new(lat), Degrees::new(lon));
                    let noise_height =
                        noise.get_noise3d(sphere_coords.0, sphere_coords.1, sphere_coords.2);
                    let n = noise_to_planet_height(noise_height);
                    if n < min {
                        min = n
                    }
                    if n > max {
                        max = n
                    }
                    max_noise = f32::max(max_noise, noise_height);
                    total_height += n as u32;
                    tile_count += 1;
                }
            }

            let pidx = planet_idx(x, y);
            planet.landblocks[pidx].height = (total_height / tile_count) as u8;
            planet.landblocks[pidx].variance = max - min;

            //let lon = noise_lon(x, 0);
            let altitude_meters = max_noise * 8_848.0; // Everest
            let temperature_decrease =
                temperature_decrease_by_altitude(f32::max(altitude_meters, 0.0));
            planet.landblocks[pidx].rainfall_mm = rainfall_mm as i32;
            planet.landblocks[pidx].temperature_c = base_temperature_c - temperature_decrease;
            planet.landblocks[pidx].air_pressure_kpa =
                atmospheric_pressure_by_elevation(altitude_meters)
                    + ((base_temperature_c - temperature_decrease) / 10.0);
        }

        if y % 8 == 0 {
            let mut bumpy_planet = PlanetMesh::new();
            bumpy_planet.with_altitude(&planet);
            PLANET_GEN.write().globe_info = Some(bumpy_planet);
        }
    }
}
