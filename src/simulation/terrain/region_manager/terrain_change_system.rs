use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use super::{build_render_chunk, process_terrain_changes, terrain_changes_requested};

/// Processes pending terrain changes and spawns update tasks
pub fn terrain_change_system(
    mut commands: Commands,
    task_master: Res<AsyncComputeTaskPool>,
) {
    if terrain_changes_requested() {
        let updates = process_terrain_changes();
        for (region_id, chunks) in updates.iter() {
            for chunk in chunks.iter() {
                let region = region_id.clone();
                let cloc = chunk.clone();
                let task = task_master.spawn(async move { build_render_chunk(region, cloc) });
                commands.spawn().insert(task);
            }
        }
    }
}