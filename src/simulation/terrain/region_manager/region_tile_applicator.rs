use super::{build_render_chunk, RegionChunkPopulator};
use crate::simulation::{
    terrain::{chunk_iter::ChunkIterator, PlanetLocation, RegionStatus, REGIONS},
    CHUNKS_PER_REGION, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH, WORLD_WIDTH,
};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;

/// Applies the result of building individual region chunks
pub fn region_tile_applicator_system(
    mut commands: Commands,
    mut region_loaders: Query<(Entity, &mut Task<RegionChunkPopulator>)>,
    task_master: Res<AsyncComputeTaskPool>,
) {
    for (task_entity, mut task) in region_loaders.iter_mut() {
        if let Some(chunk) = future::block_on(future::poll_once(&mut *task)) {
            let mut region_lock = REGIONS.write();
            let region_id = chunk.region_id;
            if let Some(region) = region_lock.regions.get_mut(&region_id) {
                let chunk_x = chunk.chunk_id.x / CHUNK_SIZE;
                let chunk_y = chunk.chunk_id.y / CHUNK_SIZE;
                let chunk_z = chunk.chunk_id.z / CHUNK_SIZE;
                let chunk_id =
                    (chunk_z * CHUNK_WIDTH * CHUNK_HEIGHT) + (chunk_y * CHUNK_WIDTH) + chunk_x;

                ChunkIterator::new(chunk.chunk_id)
                    .enumerate()
                    .for_each(|(idx, chunk_idx)| {
                        region.tile_types[chunk_idx.to_tile_index()] = chunk.tile_types[idx];
                        region.material[chunk_idx.to_tile_index()] = chunk.material[idx];
                        region.revealed[chunk_idx.to_tile_index()] = chunk.revealed[idx];
                    });

                region.chunks_loaded[chunk_id] = true;
                if region.chunks_loaded.iter().filter(|l| **l == true).count() == CHUNKS_PER_REGION
                {
                    region.status = RegionStatus::CreatedTiles;
                    println!("Created tiles");
                }

                if region.requires_render_chunks {
                    let region = chunk.region_id.clone();
                    let tile_x = region % WORLD_WIDTH;
                    let tile_y = region / WORLD_WIDTH;
                    let cloc = chunk.chunk_id.clone();
                    let task = task_master.spawn(async move {
                        build_render_chunk(PlanetLocation::new(tile_x, tile_y), cloc)
                    });
                    commands.spawn().insert(task);
                }
            } else {
                panic!("Received region chunk data for a non-loaded region");
            }

            // Remove the task now that it's done
            commands.entity(task_entity).despawn();
        }
    }
}
