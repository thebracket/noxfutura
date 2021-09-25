use bevy::{prelude::*, tasks::Task};
use futures_lite::future;
use crate::simulation::terrain::Region;
use crate::simulation::terrain::REGIONS;

/// Obtains the results of future calls to load regions, and spawns meshing
/// tasks as needed once they complete.
pub fn created_regions_handler_system(
    mut commands: Commands,
    mut region_loaders: Query<(Entity, &mut Task<Region>)>,
) {
    for (task_entity, mut task) in region_loaders.iter_mut() {
        if let Some(region) = future::block_on(future::poll_once(&mut *task)) {
            let region_id = region.location.to_region_index();
            let mut region_lock = REGIONS.write();
            region_lock.regions.remove(&region_id);
            println!("{:?}", region.status);
            region_lock.regions.insert(region_id, region);
            commands.entity(task_entity).despawn();
        }
    }
}