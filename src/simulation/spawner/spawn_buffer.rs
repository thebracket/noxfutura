use lazy_static::*;
use parking_lot::RwLock;

use crate::simulation::terrain::PlanetLocation;

lazy_static! {
    pub static ref SPAWNS: RwLock<Vec<EntitySpawn>> = RwLock::new(Vec::new());
}

pub struct EntitySpawn {
    pub region_id: PlanetLocation,
    pub tile_location: usize,
    pub event: SpawnRequest,
}

pub enum SpawnRequest {
    Tree,
}

pub fn spawn_tree(region_id: PlanetLocation, tile_location: usize) {
    SPAWNS.write().push(EntitySpawn {
        region_id,
        tile_location,
        event: SpawnRequest::Tree,
    });
}
