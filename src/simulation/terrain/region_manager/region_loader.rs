use crate::simulation::terrain::{AllChunksIterator, RegionStatus, region_chunks::build_render_chunk};
use bevy::{
    prelude::*,
    tasks::AsyncComputeTaskPool,
};
use crate::simulation::terrain::REGIONS;

/// Handles the process of loading required regions, spawning off async
/// tasks that are then handled by the created_regions_handler
/// system.
pub fn load_regions(
    mut commands: Commands,
    task_master: Res<AsyncComputeTaskPool>,
) {
    // Internal scope to remove lock at end
    {
        let mut region_lock = REGIONS.write();
        for (_region_id, region) in region_lock.regions.iter_mut() {
            match region.status {
                RegionStatus::NotLoaded => {
                    // Spawn a region loader task
                    region.status = RegionStatus::CreatingTiles;
                    let mut my_region = region.clone();
                    let task = task_master.spawn(async move {
                        my_region.build_tiles();
                        my_region
                    });

                    commands.spawn().insert(task);
                }
                RegionStatus::CreatedTiles => {
                    if region.should_rechunk {
                        region.should_rechunk = false;
                        // Spawn chunk-making tasks
                        AllChunksIterator::new().for_each(|chunk_base| {
                            let loc = region.location;
                            let cloc = chunk_base.clone();
                            let task =
                                task_master.spawn(async move { build_render_chunk(loc, cloc) });
                            commands.spawn().insert(task);
                        });
                    }
                }
                _ => {}
            }
        }
    }
}
