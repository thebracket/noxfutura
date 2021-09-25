use bevy::prelude::{Handle, Mesh};
use crate::simulation::terrain::{ChunkLocation, PlanetLocation};

pub struct MapRenderLayer {
    pub chunk_base: ChunkLocation,
    pub region_id: PlanetLocation,
    pub world_z: usize,
    pub mesh_handle: Handle<Mesh>,
}
