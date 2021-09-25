use crate::simulation::terrain::REGIONS;
use crate::simulation::terrain::{AllChunksIterator, RegionStatus};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use super::populate_region_chunk;

/// Handles the process of loading required regions, spawning off async
/// tasks that are then handled by the created_regions_handler
/// system.
pub fn load_regions(mut commands: Commands, task_master: Res<AsyncComputeTaskPool>) {
    let to_do = REGIONS
        .read()
        .regions
        .values()
        .filter(|r| r.status == RegionStatus::NotLoaded)
        .count();
    if to_do == 0 {
        return;
    }

    let mut region_lock = REGIONS.write();
    for (region_id, region) in region_lock.regions.iter_mut() {
        match region.status {
            RegionStatus::NotLoaded => {
                // Spawn a region loader task
                region.status = RegionStatus::CreatingTiles;
                AllChunksIterator::new().for_each(|chunk_base| {
                    let region = *region_id; // Copy to ensure we have a local to move
                    let cloc = chunk_base.clone(); // Ditto
                    let task =
                        task_master.spawn(async move { populate_region_chunk(region, cloc) });
                    commands.spawn().insert(task);
                });
            }
            _ => {}
        }
    }
}
