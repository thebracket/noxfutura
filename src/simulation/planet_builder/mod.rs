mod planet_3d;
use bracket_noise::prelude::*;
pub use planet_3d::PlanetMesh;
use parking_lot::RwLock;
use lazy_static::*;
use bevy::prelude::*;

lazy_static! {
    static ref PLANET_GEN : RwLock<PlanetGen> = RwLock::new(PlanetGen::new());
}

pub enum PlanetBuilderStatus {
    Initializing,
    Flattening,
    Altitudes,
}

pub struct PlanetBuilder {
    started: bool,
    pub globe_mesh_handle: Option<Handle<Mesh>>,
}

impl PlanetBuilder {
    pub fn new() -> Self {
        Self{
            started: false,
            globe_mesh_handle: None,
        }
    }

    pub fn get_status(&self) -> String {
        match PLANET_GEN.read().status {
            PlanetBuilderStatus::Initializing => String::from("Building a giant ball of mud"),
            PlanetBuilderStatus::Flattening => String::from("Smoothing out the corners"),
            PlanetBuilderStatus::Altitudes => String::from("Squishing out some topology"),
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
    status : PlanetBuilderStatus,
    globe_info: Option<PlanetMesh>,
}

impl PlanetGen {
    fn new() -> Self {
        Self{
            status : PlanetBuilderStatus::Initializing,
            globe_info: None,
        }
    }
}

fn update_status(new_status: PlanetBuilderStatus) {
    PLANET_GEN.write().status = new_status;
}

fn make_planet() {
    update_status(PlanetBuilderStatus::Initializing);
    let mut planet = Planet{
        rng_seed: 1,
        noise_seed: 1,
        landblocks: Vec::new(),
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
    println!("Type Allocation");
    println!("Coastlines");
    println!("Rainfall");
    println!("Biomes");
    println!("Rivers");
}

pub struct Planet {
    pub rng_seed: u64,
    pub noise_seed: u64,
    pub landblocks: Vec<Landblock>,
}

pub struct Landblock {
    pub height: u8,
    pub variance: u8,
    pub btype: BlockType,
    pub temperature: i8,
    pub rainfall: i8,
    pub biome_idx: usize,
}

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
            planet.landblocks.push(Landblock{
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
    const SUB_SAMPLES : i32 = 4;
    const SAMPLE_STEP : f32 = 1.0 / SUB_SAMPLES as f32;

    let mut noise = FastNoise::seeded(planet.noise_seed);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(5);
    noise.set_fractal_gain(0.5);
    noise.set_fractal_lacunarity(2.0);
    noise.set_frequency(0.01);

    for y in 0..WORLD_HEIGHT {
        let base_lon = ((y as f32 / WORLD_HEIGHT as f32) * 360.0) - 180.0;
        for x in 0..WORLD_WIDTH {
            let base_lat = ((x as f32 / WORLD_WIDTH as f32) * 180.0) - 90.0;

            let mut total_height = 0u32;
            let mut tile_count = 0u32;
            let mut max = 0;
            let mut min = std::u8::MAX;
            for lon_step in 0..SUB_SAMPLES {
                let lon = base_lon + (lon_step as f32 * SAMPLE_STEP);
                for lat_step in 0..SUB_SAMPLES {
                    let lat = base_lat + (lat_step as f32 * SAMPLE_STEP);
                    let sphere_coords = sphere_vertex(100.0, Degrees::new(lat), Degrees::new(lon));
                    let noise_height = noise.get_noise3d(sphere_coords.0, sphere_coords.1, sphere_coords.2);
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

        let mut bumpy_planet = PlanetMesh::new();
        bumpy_planet.with_altitude(&planet);
        PLANET_GEN.write().globe_info = Some(bumpy_planet);
    }
}