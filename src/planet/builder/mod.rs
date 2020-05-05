use parking_lot::Mutex;
use super::{Planet, Block, BlockType};
use bracket_noise::prelude::*;

#[derive(Clone)]
pub struct PlanetParams {
    pub world_seed: i32,
    pub water_level: i32,
    pub plains_level: i32,
    pub starting_settlers: i32,
    pub strict_beamdown : bool
}

struct PlanetBuilder {
    params : PlanetParams,
    planet : Planet,
    done : bool,
    task : String
}

impl PlanetBuilder {
    fn new() -> Self {
        Self {
            params : PlanetParams{
                world_seed: 0,
                water_level : 3,
                plains_level : 3,
                starting_settlers: 6,
                strict_beamdown: true,
            },
            planet : Planet::new(),
            done : false,
            task : "Initializing".to_string()
        }
    }
}

lazy_static! {
    static ref PLANET_BUILD : Mutex<PlanetBuilder> = Mutex::new(PlanetBuilder::new());
}

pub fn start_building_planet(params : PlanetParams) {
    let mut lock = PLANET_BUILD.lock();
    lock.planet.rng_seed = params.world_seed as u64;
    lock.planet.water_divisor = params.water_level;
    lock.planet.plains_divisor = params.plains_level;
    lock.planet.starting_settlers = params.starting_settlers;
    lock.planet.strict_beamdown = params.strict_beamdown;
    lock.params = params;
    std::mem::drop(lock);
    std::thread::spawn(threaded_builder);
}

fn threaded_builder() {
    zero_fill();
    planetary_noise();
    planet_type_allocation();
    planet_coastlines();
    planet_rainfall();
    // Biomes
    // Rivers
    // History
    // Save
    // Find crash site
    // Materialize region
    // It's all done
}

fn zero_fill() {
    use super::WORLD_TILES_COUNT;
    set_worldgen_status("Building initial ball of mud");
    let blocks : Vec<Block> = vec![Block::blank(); WORLD_TILES_COUNT as usize];
    PLANET_BUILD.lock().planet.landblocks = blocks;
    PLANET_BUILD.lock().planet.migrant_counter = 0;
    PLANET_BUILD.lock().planet.remaining_settlers = 0;
}

fn planetary_noise() {
    use super::{WORLD_HEIGHT, WORLD_WIDTH, REGION_HEIGHT, REGION_WIDTH};
    set_worldgen_status("Dividing the heavens from the earth");
    let perlin_seed = PLANET_BUILD.lock().planet.perlin_seed;
    let mut noise = FastNoise::seeded(perlin_seed);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(5);
    noise.set_fractal_gain(0.5);
    noise.set_fractal_lacunarity(2.0);

    let max_temperature = 56.7;
    let min_temperature = -55.2;
    let temperature_range = max_temperature - min_temperature;
    let half_planet_height = WORLD_HEIGHT as f32 / 2.0;
    const REGION_FRACTION_TO_CONSIDER : i32 = 64;

    for y in 0..WORLD_HEIGHT as i32 {
        let distance_from_equator = i32::abs((WORLD_HEIGHT as i32 / 2) - y);
        let temp_range_percent = 1.0 - distance_from_equator as f32 / half_planet_height;
        let base_temp_by_latitude = (temp_range_percent * temperature_range) + min_temperature;
        for x in 0..WORLD_WIDTH as i32 {
            let mut total_height = 0u32;

            let mut max = 0;
            let mut min = std::u8::MAX;
            let mut n_tiles = 0;
            for y1 in 0..REGION_HEIGHT / REGION_FRACTION_TO_CONSIDER {
                for x1 in 0..REGION_WIDTH / REGION_FRACTION_TO_CONSIDER {
                    let nh = noise.get_noise(
                        noise_x(x, x1*REGION_FRACTION_TO_CONSIDER),
                        noise_y(y, y1*REGION_FRACTION_TO_CONSIDER)
                    );
                    let n = noise_to_planet_height(nh);
                    if n < min { min = n }
                    if n > max { max = n }
                    total_height += n as u32;
                    n_tiles += 1;
                }
            }

            let pidx = super::planet_idx(x, y);
            let mut planet = PLANET_BUILD.lock();
            planet.planet.landblocks[pidx].height = (total_height / n_tiles as u32) as u8;
            planet.planet.landblocks[pidx].btype = BlockType::None;
            planet.planet.landblocks[pidx].variance = max - min;
            let altitude_deduction = (planet.planet.landblocks[pidx].height as f32 - planet.planet.water_height as f32) / 10.0;
            planet.planet.landblocks[pidx].temperature = (base_temp_by_latitude - altitude_deduction) as i8;
            if planet.planet.landblocks[pidx].temperature < -55 { planet.planet.landblocks[pidx].temperature = -55 }
            if planet.planet.landblocks[pidx].temperature > 55 { planet.planet.landblocks[pidx].temperature = 55 }
            std::mem::drop(planet);
        }

        let percent = y as f32 / WORLD_HEIGHT as f32;
        set_worldgen_status(format!("Dividing heavens from the earth: {}%", (percent * 100.0) as u8));
    }
}

const NOISE_SIZE : f32 = 384.0;

fn noise_x(world_x: i32, region_x: i32) -> f32 {
    use super::{WORLD_WIDTH, REGION_WIDTH};
    let big_x = ((world_x * WORLD_WIDTH as i32) + region_x) as f32;
    (big_x / WORLD_WIDTH as f32 * REGION_WIDTH as f32) * NOISE_SIZE
}

fn noise_y(world_y : i32, region_y : i32) -> f32 {
    use super::{WORLD_HEIGHT, REGION_HEIGHT};
    let big_y = ((world_y * WORLD_HEIGHT as i32) + region_y) as f32;
    (big_y / WORLD_HEIGHT as f32 * REGION_HEIGHT as f32) * NOISE_SIZE
}

fn noise_to_planet_height(n: f32) -> u8 {
    ((n + 1.0) * 150.0) as u8
}

fn planet_determine_proportion(planet: &Planet, candidate: &mut i32, target: i32) -> u8 {
    let mut count = 0usize;
    while count < target as usize {
        count = planet.landblocks.iter().filter(|b| b.height < *candidate as u8).count();
        if count >= target as usize {
            return *candidate as u8;
        } else {
            *candidate += 1;
        }
    }
    0
}

fn planet_type_allocation() {
    use super::WORLD_TILES_COUNT;
    set_worldgen_status("Dividing the waters from the earth");
    let mut candidate = 0;
    let mut planet = PLANET_BUILD.lock().planet.clone();
    let remaining_divisor = 10 - (planet.water_divisor + planet.plains_divisor);
    let n_cells = WORLD_TILES_COUNT as i32;
    let n_cells_water = n_cells / planet.water_divisor;
    let n_cells_plains = (n_cells / planet.plains_divisor) + n_cells_water;
    let n_cells_hills = (n_cells / remaining_divisor) + n_cells_plains;

    planet.water_height = planet_determine_proportion(&mut planet, &mut candidate, n_cells_water);
    planet.plains_height = planet_determine_proportion(&mut planet, &mut candidate, n_cells_plains);
    planet.hills_height = planet_determine_proportion(&mut planet, &mut candidate, n_cells_hills);

    for block in planet.landblocks.iter_mut() {
        if block.height <= planet.water_height {
            block.btype = BlockType::Water;
            block.rainfall = 10;
        }
        if block.height + block.variance/2 > planet.water_height {
            block.btype = BlockType::SaltMarsh;
        } else if block.height <= planet.plains_height {
            block.btype = BlockType::Plains;
            block.rainfall = 10;
            if block.height - block.variance/2 > planet.water_height {
                block.btype = BlockType::Marsh;
                block.rainfall = 20;
            }
        } else if block.height <= planet.hills_height {
            block.btype = BlockType::Hills;
            block.rainfall = 20;
            if block.variance < 2 {
                block.btype = BlockType::Highlands;
                block.rainfall = 10;
            }
        } else {
            block.btype = BlockType::Mountains;
            block.rainfall = 30;
            if block.variance < 3 {
                block.btype = BlockType::Plateau;
                block.rainfall = 10;
            }
        }
    }

    PLANET_BUILD.lock().planet.landblocks = planet.landblocks;
}

fn planet_coastlines() {
    use super::{WORLD_WIDTH, WORLD_HEIGHT, planet_idx};
    set_worldgen_status("Crinkling the coastlines");
    let mut planet = PLANET_BUILD.lock().planet.clone();

    for y in 1..WORLD_HEIGHT as i32 -1 {
        for x in 1..WORLD_WIDTH as i32 - 1 {
            let base_idx = planet_idx(x, y);
            if planet.landblocks[base_idx].btype != BlockType::Water {
                if planet.landblocks[base_idx - 1].btype == BlockType::Water ||
                    planet.landblocks[base_idx + 1].btype == BlockType::Water ||
                    planet.landblocks[base_idx - WORLD_WIDTH as usize].btype == BlockType::Water ||
                    planet.landblocks[base_idx + WORLD_WIDTH as usize].btype == BlockType::Water 
                    {
                        planet.landblocks[base_idx].btype = BlockType::Coastal;
                        planet.landblocks[base_idx].rainfall = 20;
                    }
            }
        }
    }

    PLANET_BUILD.lock().planet.landblocks = planet.landblocks;
}

fn planet_rainfall() {
    use super::{WORLD_WIDTH, WORLD_HEIGHT, planet_idx};
    set_worldgen_status("And then it rained a lot");
    let mut planet = PLANET_BUILD.lock().planet.clone();
    for y in 0..WORLD_HEIGHT as i32 {
        let mut rain_amount = 10;
        for x in 0..WORLD_WIDTH as i32 {
            let pidx = planet_idx(x, y);
            if planet.landblocks[pidx].btype == BlockType::Mountains {
                rain_amount -= 20;
            } else if planet.landblocks[pidx].btype == BlockType::Hills {
                rain_amount -= 10;
            } else if planet.landblocks[pidx].btype == BlockType::Coastal {
                rain_amount -= 5;
            } else {
                rain_amount += 1;
            }
            if rain_amount < 0 { rain_amount = 0; }
            if rain_amount > 20 { rain_amount = 20; }
            planet.landblocks[pidx].rainfall += rain_amount;
            if planet.landblocks[pidx].rainfall < 0 { planet.landblocks[pidx].rainfall = 0 }
            if planet.landblocks[pidx].rainfall > 100 { planet.landblocks[pidx].rainfall = 100 }
        }
    }
    PLANET_BUILD.lock().planet.landblocks = planet.landblocks;
}

fn set_worldgen_status<S:ToString>(status : S) {
    PLANET_BUILD.lock().task = status.to_string();
}

pub fn get_worldgen_status() -> String {
    PLANET_BUILD.lock().task.clone()
}