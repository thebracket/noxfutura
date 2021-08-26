mod planet_3d;
use bevy::prelude::*;
use bracket_noise::prelude::*;
use lazy_static::*;
use parking_lot::RwLock;
pub use planet_3d::PlanetMesh;

lazy_static! {
    static ref PLANET_GEN: RwLock<PlanetGen> = RwLock::new(PlanetGen::new());
}

pub enum PlanetBuilderStatus {
    Initializing,
    Flattening,
    Altitudes,
    Dividing,
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

    println!("Coastlines");
    println!("Rainfall");
    println!("Biomes");
    println!("Rivers");
}

pub struct Planet {
    pub rng_seed: u64,
    pub noise_seed: u64,
    pub landblocks: Vec<Landblock>,
    pub water_height: u8,
    pub plains_height: u8,
    pub hills_height: u8,
}

pub struct Landblock {
    pub height: u8,
    pub variance: u8,
    pub btype: BlockType,
    pub temperature: i8,
    pub rainfall: i8,
    pub biome_idx: usize,
}

#[derive(Clone, Copy)]
pub enum BlockType {
    None,
    Water,
    Plains,
    Hills,
    Mountains,
    Marsh,
    Plateau,
    Highlands,
    Coastal,
    SaltMarsh,
}

use crate::{geometry::Degrees, simulation::planet_builder::planet_3d::sphere_vertex};

use super::bounds::*;

fn zero_fill(planet: &mut Planet) {
    for y in 0..WORLD_HEIGHT {
        for x in 0..WORLD_WIDTH {
            planet.landblocks.push(Landblock {
                height: 0,
                variance: 0,
                btype: BlockType::None,
                temperature: 0,
                rainfall: 0,
                biome_idx: planet_idx(x, y),
            });
        }
    }
}

fn noise_to_planet_height(n: f32) -> u8 {
    ((n + 1.0) * 150.0) as u8
}

fn planetary_noise(planet: &mut Planet) {
    const SAMPLE_DIVISOR: usize = 32;
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
        for x in 0..WORLD_WIDTH {
            let mut total_height = 0u32;
            let mut tile_count = 0u32;
            let mut max = 0;
            let mut min = std::u8::MAX;
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
                    total_height += n as u32;
                    tile_count += 1;
                }
            }

            let pidx = planet_idx(x, y);
            planet.landblocks[pidx].height = (total_height / tile_count) as u8;
            planet.landblocks[pidx].variance = max - min;
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
            block.rainfall = 10;

            if block.height as u16 + block.variance as u16 / 2 > planet.water_height as u16 {
                block.btype = BlockType::SaltMarsh;
            }
        } else if block.height <= planet.plains_height {
            block.btype = BlockType::Plains;
            block.rainfall = 10;
            // TODO: Fix me
            /*if block.height - block.variance / 3 > planet.water_height {
                block.btype = BlockType::Marsh;
                block.rainfall = 20;
            }*/
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

        if i % 500 == 0 {
            let mut bumpy_planet = PlanetMesh::new();
            bumpy_planet.with_category(&planet);
            PLANET_GEN.write().globe_info = Some(bumpy_planet);
        }
    }
}

pub(crate) fn planet_determine_proportion(planet: &Planet, candidate: &mut i32, target: i32) -> u8 {
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
