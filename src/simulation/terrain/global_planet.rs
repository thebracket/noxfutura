use crate::simulation::Planet;
use crate::{asset_handlers::vox::VoxTemplate, raws::StrataMaterials};
use bevy::prelude::{Handle, Mesh, StandardMaterial};
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
    pub grass_handle: Option<Handle<StandardMaterial>>,
    pub tree_handle: Option<Handle<Mesh>>,
    pub tree_mat: Option<Handle<StandardMaterial>>,
    pub vox_mat: Option<Handle<StandardMaterial>>,
    pub vox_templates: Vec<VoxTemplate>,
    pub vox_meshes: Vec<Handle<Mesh>>,
}

impl PlanetData {
    pub fn new() -> Self {
        Self {
            planet: None,
            strata: None,
            height_noise: None,
            material_noise: None,
            world_material_handle: None,
            grass_handle: None,
            tree_handle: None,
            tree_mat: None,
            vox_mat: None,
            vox_templates: Vec::new(),
            vox_meshes: Vec::new(),
        }
    }
}

pub fn set_global_planet(planet: Planet) {
    let planet_copy = planet.clone();
    PLANET_STORE.write().planet = Some(planet);
    PLANET_STORE.write().height_noise = Some(planet_copy.get_height_noise());
    PLANET_STORE.write().material_noise = Some(planet_copy.get_material_noise());
}
