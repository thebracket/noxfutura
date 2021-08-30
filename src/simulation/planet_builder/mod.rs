mod biomes;
mod calc;
mod coast;
mod noise;
mod planet_3d;
mod type_allocation;
mod wind_and_rain;
mod zero;

use calc::*;

use crate::{
    raws::BlockType,
    simulation::{Direction, Landblock, Planet, WORLD_TILES_COUNT},
};
use bevy::prelude::*;
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

    pub fn start(&mut self, seed: &str) {
        if !self.started {
            let seed = seed.to_string();
            std::thread::spawn(|| make_planet(seed));
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

fn make_planet(seed: String) {
    let mut base_seed = 0;
    seed.chars().for_each(|c| base_seed += c as u64);
    update_status(PlanetBuilderStatus::Initializing);
    let mut planet = Planet {
        rng_seed: base_seed + 1,
        noise_seed: base_seed,
        landblocks: Vec::with_capacity(WORLD_TILES_COUNT),
        water_height: 0,
        plains_height: 0,
        hills_height: 0,
    };

    update_status(PlanetBuilderStatus::Flattening);
    println!("Zero Fill");
    {
        zero::zero_fill(&mut planet);
        let mut round_planet = PlanetMesh::new();
        round_planet.totally_round(3.0);
        PLANET_GEN.write().globe_info = Some(round_planet);
    }

    update_status(PlanetBuilderStatus::Altitudes);
    println!("Planetary Noise");
    {
        noise::planetary_noise(&mut planet);
        let mut bumpy_planet = PlanetMesh::new();
        bumpy_planet.with_altitude(&planet);
        PLANET_GEN.write().globe_info = Some(bumpy_planet);
    }

    update_status(PlanetBuilderStatus::Dividing);
    println!("Type Allocation");
    {
        type_allocation::planet_type_allocation(&mut planet);
        let mut bumpy_planet = PlanetMesh::new();
        bumpy_planet.with_category(&planet);
        PLANET_GEN.write().globe_info = Some(bumpy_planet);
    }

    update_status(PlanetBuilderStatus::Coast);
    println!("Coastlines");
    {
        coast::planet_coastlines(&mut planet);
        let mut bumpy_planet = PlanetMesh::new();
        bumpy_planet.with_category(&planet);
        PLANET_GEN.write().globe_info = Some(bumpy_planet);
    }

    println!("Wind and rain");
    update_status(PlanetBuilderStatus::Rainfall { amount: 0 });
    wind_and_rain::planet_rainfall(&mut planet);

    println!("Biomes");
    update_status(PlanetBuilderStatus::Biomes);
    {
        biomes::planet_biomes(&mut planet);
        let mut bumpy_planet = PlanetMesh::new();
        bumpy_planet.with_biomes(&planet);
        PLANET_GEN.write().globe_info = Some(bumpy_planet);
    }

    println!("Saving...");
}
