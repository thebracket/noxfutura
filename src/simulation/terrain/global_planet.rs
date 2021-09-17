use super::strata::StrataMaterials;
use crate::simulation::Planet;
use bevy::prelude::{Handle, StandardMaterial};
use bracket_noise::prelude::FastNoise;
use lazy_static::*;
use parking_lot::RwLock;

lazy_static! {
    pub static ref PLANET_STORE: RwLock<PlanetData> = RwLock::new(PlanetData::new());
}

pub struct PlanetData {
    pub planet: Option<Planet>,
    pub strata: Option<StrataMaterials>,
    pub height_noise: Option<FastNoise>,
    pub material_noise: Option<FastNoise>,
    pub world_material_handle: Option<Vec<Handle<StandardMaterial>>>,
}

impl PlanetData {
    pub fn new() -> Self {
        Self {
            planet: None,
            strata: None,
            height_noise: None,
            material_noise: None,
            world_material_handle: None,
        }
    }
}
