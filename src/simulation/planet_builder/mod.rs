mod planet_3d;
use crate::raws::BlockType;
use bevy::prelude::*;
use bracket_noise::prelude::*;
use lazy_static::*;
use parking_lot::RwLock;
pub use planet_3d::PlanetMesh;
use std::collections::HashSet;

lazy_static! {
    static ref PLANET_GEN: RwLock<PlanetGen> = RwLock::new(PlanetGen::new());
}

pub enum PlanetBuilderStatus {
    Initializing,
    Flattening,
    Altitudes,
    Dividing,
    Coast,
    Rainfall { amount: u8 },
    Biomes,
}

pub struct PlanetBuilder {
    started: bool,
    pub globe_mesh_handle: Option<Handle<Mesh>>,
}

impl PlanetBuilder {
    pub fn new() -> Self {
        Self {
            started: false,
            globe_mesh_handle: None,
        }
    }

    pub fn get_status(&self) -> String {
        match PLANET_GEN.read().status {
            PlanetBuilderStatus::Initializing => String::from("Building a giant ball of mud"),
            PlanetBuilderStatus::Flattening => String::from("Smoothing out the corners"),
            PlanetBuilderStatus::Altitudes => String::from("Squishing out some topology"),
            PlanetBuilderStatus::Dividing => String::from("Dividing the heaven and hearth"),
            PlanetBuilderStatus::Coast => String::from("Crinkling up the coastlines"),
            PlanetBuilderStatus::Rainfall { amount } => {
                format!("Spinning the barometer {}%", amount)
            }
            PlanetBuilderStatus::Biomes => String::from("Zooming on on details"),
        }
    }

    pub fn start(&mut self) {
        if !self.started {
            std::thread::spawn(|| make_planet());
            self.started = true;
        }
    }

    pub fn globe_info(&self) -> Option<PlanetMesh> {
        let has_info = PLANET_GEN.read().globe_info.is_some();
        if has_info {
            let mut write_lock = PLANET_GEN.write();
            write_lock.globe_info.take()
        } else {
            None
        }
    }
}

struct PlanetGen {
    status: PlanetBuilderStatus,
    globe_info: Option<PlanetMesh>,
}

impl PlanetGen {
    fn new() -> Self {
        Self {
            status: PlanetBuilderStatus::Initializing,
            globe_info: None,
        }
    }
}

fn update_status(new_status: PlanetBuilderStatus) {
    PLANET_GEN.write().status = new_status;
}

fn make_planet() {
    update_status(PlanetBuilderStatus::Initializing);
    let mut planet = Planet {
        rng_seed: 1,
        noise_seed: 5,
        landblocks: Vec::new(),
        water_height: 0,
        plains_height: 0,
        hills_height: 0,
    };

    update_status(PlanetBuilderStatus::Flattening);
    println!("Zero Fill");
    {
        zero_fill(&mut planet);
        let mut round_planet = PlanetMesh::new();
        round_planet.totally_round(3.0);
        PLANET_GEN.write().globe_info = Some(round_planet);
    }

    update_status(PlanetBuilderStatus::Altitudes);
    println!("Planetary Noise");
    {
        planetary_noise(&mut planet);
        let mut bumpy_planet = PlanetMesh::new();
        bumpy_planet.with_altitude(&planet);
        PLANET_GEN.write().globe_info = Some(bumpy_planet);
    }

    update_status(PlanetBuilderStatus::Dividing);
    println!("Type Allocation");
    {
        planet_type_allocation(&mut planet);
        let mut bumpy_planet = PlanetMesh::new();
        bumpy_planet.with_category(&planet);
        PLANET_GEN.write().globe_info = Some(bumpy_planet);
    }

    update_status(PlanetBuilderStatus::Coast);
    println!("Coastlines");
    {
        planet_coastlines(&mut planet);
        let mut bumpy_planet = PlanetMesh::new();
        bumpy_planet.with_category(&planet);
        PLANET_GEN.write().globe_info = Some(bumpy_planet);
    }

    println!("Wind and rain");
    update_status(PlanetBuilderStatus::Rainfall { amount: 0 });
    planet_rainfall(&mut planet);

    println!("Biomes");
    update_status(PlanetBuilderStatus::Biomes);
    planet_biomes(&mut planet);
}

pub struct Planet {
    pub rng_seed: u64,
    pub noise_seed: u64,
    pub landblocks: Vec<Landblock>,
    pub water_height: u8,
    pub plains_height: u8,
    pub hills_height: u8,
}

#[derive(Clone, Debug)]
pub struct Landblock {
    pub height: u8,
    pub variance: u8,
    pub btype: BlockType,
    pub temperature_c: f32,
    pub rainfall_mm: i32,
    pub air_pressure_kpa: f32,
    pub prevailing_wind: Direction,
    pub biome_idx: usize,
    pub neighbors: [(Direction, usize); 4],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    None,
}

use crate::{
    geometry::{Degrees, Radians},
    simulation::planet_builder::planet_3d::sphere_vertex,
};

use super::bounds::*;

fn zero_fill(planet: &mut Planet) {
    for y in 0..WORLD_HEIGHT {
        for x in 0..WORLD_WIDTH {
            planet.landblocks.push(Landblock {
                height: 0,
                variance: 0,
                btype: BlockType::None,
                temperature_c: 0.0,
                rainfall_mm: 0,
                biome_idx: usize::MAX,
                air_pressure_kpa: 0.0,
                prevailing_wind: Direction::None,
                neighbors: planet_neighbors_four_way(planet_idx(x, y)),
            });
        }
    }
}

fn noise_to_planet_height(n: f32) -> u8 {
    ((n + 1.0) * 150.0) as u8
}

fn planetary_noise(planet: &mut Planet) {
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

fn planet_type_allocation(planet: &mut Planet) {
    const WATER_DIVISOR: usize = 3;
    const PLAINS_DIVISOR: usize = 3;
    const REMAINING_DIVISOR: usize = 10 - (WATER_DIVISOR + PLAINS_DIVISOR);
    let n_cells = WORLD_TILES_COUNT;
    let n_cells_water = n_cells / WATER_DIVISOR;
    let n_cells_plains = (n_cells / PLAINS_DIVISOR) + n_cells_water;
    let n_cells_hills = (n_cells / REMAINING_DIVISOR) + n_cells_plains;

    let mut candidate = 0;
    planet.water_height = planet_determine_proportion(planet, &mut candidate, n_cells_water as i32);
    planet.plains_height =
        planet_determine_proportion(planet, &mut candidate, n_cells_plains as i32);
    planet.hills_height = planet_determine_proportion(planet, &mut candidate, n_cells_hills as i32);

    for i in 0..planet.landblocks.len() {
        let mut block = &mut planet.landblocks[i];
        if block.height <= planet.water_height {
            block.btype = BlockType::Water;

            if block.height as u16 + block.variance as u16 / 2 > planet.water_height as u16 {
                block.btype = BlockType::SaltMarsh;
            }
        } else if block.height <= planet.plains_height {
            block.btype = BlockType::Plains;
            if block.height - block.variance < planet.water_height {
                block.btype = BlockType::Marsh;
            }
        } else if block.height <= planet.hills_height {
            block.btype = BlockType::Hills;
            if block.variance < 2 {
                block.btype = BlockType::Highlands;
            }
        } else {
            block.btype = BlockType::Mountains;
            if block.variance < 3 {
                block.btype = BlockType::Plateau;
            }
        }

        if i % 500 == 0 {
            let mut bumpy_planet = PlanetMesh::new();
            bumpy_planet.with_category(&planet);
            PLANET_GEN.write().globe_info = Some(bumpy_planet);
        }
    }
}

fn planet_determine_proportion(planet: &Planet, candidate: &mut i32, target: i32) -> u8 {
    let mut count = 0usize;
    while count < target as usize {
        count = planet
            .landblocks
            .iter()
            .filter(|b| b.height <= *candidate as u8)
            .count();
        if count >= target as usize {
            return *candidate as u8;
        } else {
            *candidate += 1;
        }
    }
    0
}

fn planet_coastlines(planet: &mut Planet) {
    let mut n = 0;
    for y in 1..WORLD_HEIGHT - 1 {
        for x in 1..WORLD_WIDTH - 1 {
            let base_idx = planet_idx(x, y);
            if planet.landblocks[base_idx].btype != BlockType::Water {
                if planet.landblocks[base_idx - 1].btype == BlockType::Water
                    || planet.landblocks[base_idx + 1].btype == BlockType::Water
                    || planet.landblocks[base_idx - WORLD_WIDTH as usize].btype == BlockType::Water
                    || planet.landblocks[base_idx + WORLD_WIDTH as usize].btype == BlockType::Water
                {
                    planet.landblocks[base_idx].btype = BlockType::Coastal;
                    n += 1;
                    if n % 1000 == 0 {
                        let mut bumpy_planet = PlanetMesh::new();
                        bumpy_planet.with_category(&planet);
                        PLANET_GEN.write().globe_info = Some(bumpy_planet);
                    }
                }
            }
        }
    }
}

fn average_temperature_by_latitude(lat: Degrees) -> f32 {
    // Source: https://davidwaltham.com/global-warming-model/
    const AVERAGE_EQUATORIAL_C: f32 = 30.0;
    const A: f32 = 35.0; // Based on current data
    let lat_rad: Radians = lat.into();
    let lat_sin_squared = lat_rad.0.sin() * lat_rad.0.sin();
    AVERAGE_EQUATORIAL_C - (A * lat_sin_squared)
}

fn average_precipitation_mm_by_latitude(lat: Degrees) -> f32 {
    // Mangled from https://i.stack.imgur.com/YBgot.png
    const PEAK: f32 = 8000.0;
    let fudge = if (lat.0 > -50.0 && lat.0 < -5.0) || (lat.0 < 50.0 && lat.0 > 5.0) {
        400.0
    } else {
        0.0
    };
    let lat_rad: Radians = lat.into();
    let lat_sin_squared = lat_rad.0.sin() * lat_rad.0.sin();
    PEAK - (lat_sin_squared * PEAK) - fudge
}

fn temperature_decrease_by_altitude(altitude_meters: f32) -> f32 {
    (altitude_meters / 1000.0) * 6.5
}

fn atmospheric_pressure_by_elevation(altitude_meters: f32) -> f32 {
    (101_325.0 * (1.0 - 2.25577 * 0.00001 * altitude_meters).powf(5.25588)) / 1000.0
}

fn planet_neighbors_four_way(idx: usize) -> [(Direction, usize); 4] {
    let mut result = [
        (Direction::North, 0),
        (Direction::South, 0),
        (Direction::East, 0),
        (Direction::West, 0),
    ];

    let (px, py) = idx_planet(idx);

    // West
    if px > 0 {
        result[3].1 = planet_idx(px - 1, py);
    } else {
        result[3].1 = planet_idx(WORLD_WIDTH - 1, py);
    }

    // East
    if px < WORLD_WIDTH - 1 {
        result[2].1 = planet_idx(px + 1, py);
    } else {
        result[2].1 = planet_idx(0, py);
    }

    // North
    let distance_from_middle = (WORLD_WIDTH as isize / 2) - px as isize;
    if py > 0 {
        result[0].1 = planet_idx(px, py);
    } else {
        result[0].1 = planet_idx((px as isize + distance_from_middle) as usize, py);
    }

    // South
    if py < WORLD_HEIGHT - 1 {
        result[1].1 = planet_idx(px, py);
    } else {
        result[1].1 = planet_idx((px as isize + distance_from_middle) as usize, py);
    }

    result
}

struct RainParticle {
    position: usize,
    load: i32,
    cycles: u32,
    history: HashSet<usize>,
    raining: bool,
}

impl RainParticle {
    fn take_water(&mut self, planet: &mut Planet, amount: i32) {
        if amount <= planet.landblocks[self.position].rainfall_mm {
            planet.landblocks[self.position].rainfall_mm -= amount;
            self.load += amount;
        } else {
            self.load += planet.landblocks[self.position].rainfall_mm;
            planet.landblocks[self.position].rainfall_mm = 0;
        }
    }

    fn dump_water(&mut self, planet: &mut Planet, amount: i32) {
        if self.load >= amount {
            self.load -= amount;
            planet.landblocks[self.position].rainfall_mm += amount;
        } else {
            planet.landblocks[self.position].rainfall_mm += self.load;
            self.load = 0;
        }
    }
}

fn planet_rainfall(planet: &mut Planet) {
    let lb_copy = planet.landblocks.clone();
    planet.landblocks.iter_mut().for_each(|lb| {
        let mut neighbors: Vec<(Direction, f32)> = lb
            .neighbors
            .iter()
            .map(|n| (n.0, lb_copy[n.1].air_pressure_kpa))
            //.filter(|n| n.1 <= lb.air_pressure_kpa)
            .collect();

        if neighbors.is_empty() {
            lb.prevailing_wind = Direction::None;
        } else {
            neighbors.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            lb.prevailing_wind = neighbors[0].0;
        }
    });
    let mut bumpy_planet = PlanetMesh::new();
    bumpy_planet.with_wind(&planet);
    PLANET_GEN.write().globe_info = Some(bumpy_planet);

    let mut rain_particles = Vec::with_capacity(WORLD_TILES_COUNT);
    for i in 0..planet.landblocks.len() {
        rain_particles.push(RainParticle {
            position: i,
            cycles: 0,
            load: 0,
            history: HashSet::new(),
            raining: false,
        })
    }

    while !rain_particles.is_empty() {
        rain_particles.iter_mut().for_each(|p| {
            p.cycles += 1;

            if planet.landblocks[p.position].btype == BlockType::Water {
                p.take_water(planet, 20);
            } else {
                if p.raining {
                    p.dump_water(planet, 5);
                } else {
                    p.take_water(planet, 200);
                }
            }

            if p.load < 1 {
                p.raining = false;
            }
            if p.load > 0
                && (planet.landblocks[p.position].btype == BlockType::Mountains
                    || planet.landblocks[p.position].btype == BlockType::Highlands)
            {
                p.raining = true;
            }

            let wind = planet.landblocks[p.position].prevailing_wind;
            if wind != Direction::None {
                let destination = match wind {
                    Direction::North => planet.landblocks[p.position].neighbors[0].1,
                    Direction::South => planet.landblocks[p.position].neighbors[1].1,
                    Direction::East => planet.landblocks[p.position].neighbors[2].1,
                    Direction::West => planet.landblocks[p.position].neighbors[3].1,
                    Direction::None => 0,
                };

                if !p.history.contains(&destination) {
                    p.history.insert(p.position);
                    p.position = destination;
                } else {
                    p.cycles += 500;
                }
            } else {
                p.cycles += 500;
            }
        });

        rain_particles.retain(|p| p.cycles < WORLD_WIDTH as u32 * 2);
        let percent =
            ((1.0 - (rain_particles.len() as f32 / WORLD_TILES_COUNT as f32)) * 100.0) as u8;
        update_status(PlanetBuilderStatus::Rainfall { amount: percent });
        //println!("Particles remaining: {}", rain_particles.len());
    }
}

fn planet_biomes(planet: &mut Planet) {
    use crate::raws::{BiomeType, RAWS};
    use bracket_random::prelude::RandomNumberGenerator;
    let biome_reader = RAWS.read();
    let mut rng = RandomNumberGenerator::seeded(planet.rng_seed);
    for i in 0..planet.landblocks.len() {
        let lb = &planet.landblocks[i];
        let possible_biomes: Vec<(usize, &BiomeType)> = biome_reader
            .biomes
            .areas
            .iter()
            .enumerate()
            .filter(|b| b.1.occurs.contains(&lb.btype))
            .filter(|b| {
                lb.temperature_c >= b.1.min_temp as f32 && lb.temperature_c < b.1.max_temp as f32
            })
            .filter(|b| {
                lb.rainfall_mm >= b.1.min_rain as i32 && lb.rainfall_mm < b.1.max_rain as i32
            })
            .collect();

        if possible_biomes.is_empty() {
            panic!("No biomes for {:#?}", lb);
        } else {
            if let Some(choice) = rng.random_slice_entry(&possible_biomes) {
                planet.landblocks[i].biome_idx = choice.0;
                //println!("Selected: {:?} : {}", planet.landblocks[i].btype, choice.1.name);
            }
        }

        // Render Result
        if i % 100 == 0 {
            let mut bumpy_planet = PlanetMesh::new();
            bumpy_planet.with_biomes(&planet);
            PLANET_GEN.write().globe_info = Some(bumpy_planet);
        }
    }
}
