use bevy::prelude::{Handle, Mesh};
use crate::simulation::terrain::{ChunkLocation, PlanetLocation};

/// Used as a component, attached to Mesh elements that make up the terrain.
/// Contains the indexing required to remove/replace/edit meshes.
pub struct MapRenderLayer {
    pub chunk_base: ChunkLocation,
    pub region_id: PlanetLocation,
    pub world_z: usize,
    pub mesh_handle: Handle<Mesh>,
}
