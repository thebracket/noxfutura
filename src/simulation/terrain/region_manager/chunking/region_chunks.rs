use super::{render_chunk_layer::RenderChunkLayer, RenderChunk};
use crate::simulation::terrain::REGIONS;
use crate::simulation::{
    terrain::{chunk_iter::ChunkIterator, ChunkLocation, PlanetLocation, TileType},
    CHUNK_SIZE,
};

pub fn build_render_chunk(region_id: PlanetLocation, location: ChunkLocation) -> RenderChunk {
    if is_chunk_empty(region_id, location) {
        RenderChunk::empty(region_id, location)
    } else {
        let mut chunk = RenderChunk {
            region: region_id,
            location,
            layers: Some(Vec::with_capacity(CHUNK_SIZE)),
        };

        // Build the basic geometric elements
        let region_lock = REGIONS.read();
        let region_idx = region_id.to_region_index();
        if let Some(region) = region_lock.regions.get(&region_idx) {
            for layer in 0..CHUNK_SIZE {
                let mut local_location = location;
                local_location.z += layer;
                chunk
                    .layers
                    .as_mut()
                    .unwrap()
                    .push(RenderChunkLayer::new(region_id, local_location));
            }

            ChunkIterator::new(location)
                .map(|loc| (loc, loc.to_tile_index()))
                .filter(|(_, idx)| region.revealed[*idx])
                .filter(|(_, idx)| region.tile_types[*idx] != TileType::Empty)
                .for_each(|(loc, idx)| {
                    let material = region.material[idx];
                    let layer = loc.z - location.z;
                    match region.tile_types[idx] {
                        TileType::SemiMoltenRock => {
                            chunk.layers.as_mut().unwrap()[layer].add_cube(material, idx)
                        }
                        TileType::Solid => {
                            chunk.layers.as_mut().unwrap()[layer].add_cube(material, idx)
                        }
                        TileType::Floor => {
                            chunk.layers.as_mut().unwrap()[layer].add_floor(material, idx)
                        }
                        TileType::Ramp { direction } => {
                            chunk.layers.as_mut().unwrap()[layer].add_ramp(material, idx, direction)
                        }
                        _ => {}
                    }
                });

            for layer in chunk.layers.as_mut().unwrap().iter_mut() {
                let mesh_list = layer.create_geometry();
                if mesh_list.is_empty() {
                    layer.meshes = None;
                } else {
                    layer.meshes = Some(mesh_list);
                }
            }

            chunk
        } else {
            println!("Returning none due to region inaccessible");
            RenderChunk::empty(region_id, location)
        }
    }
}

fn is_chunk_empty(region: PlanetLocation, location: ChunkLocation) -> bool {
    let region_lock = REGIONS.read();
    let region_id = region.to_region_index();
    if let Some(region) = region_lock.regions.get(&region_id) {
        let visible_chunks = ChunkIterator::new(location)
            .map(|loc| loc.to_tile_index())
            .filter(|idx| region.revealed[*idx])
            .filter(|idx| region.tile_types[*idx] != TileType::Empty)
            .count();
        if visible_chunks > 0 {
            return false;
        }
    }
    true
}
