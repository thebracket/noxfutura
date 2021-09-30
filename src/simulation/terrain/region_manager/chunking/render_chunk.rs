use super::render_chunk_layer::RenderChunkLayer;
use crate::components::PlanetLocation;
use crate::simulation::terrain::ChunkLocation;

/// A RenderChunk represents a single, renderable chunk. To support
/// Dwarf Fortress layer-cake maps, each layer is represented by its
/// own set of meshes (divided into materials).
pub struct RenderChunk {
    pub region: PlanetLocation,
    pub location: ChunkLocation,
    pub layers: Option<Vec<RenderChunkLayer>>,
}

impl RenderChunk {
    pub(crate) fn empty(region: PlanetLocation, location: ChunkLocation) -> Self {
        Self {
            region,
            location,
            layers: None,
        }
    }
}
