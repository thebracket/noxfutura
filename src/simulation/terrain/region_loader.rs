use super::{REGIONS, Region, RegionStatus, ChunkLocation, region_chunks::{RenderChunk, build_render_chunk}};
use crate::simulation::terrain::PLANET_STORE;
use crate::simulation::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;

pub fn load_regions(
    mut commands: Commands,
    task_master: Res<AsyncComputeTaskPool>,
    mut region_loaders: Query<(Entity, &mut Task<Region>)>,
    mut chunk_loaders: Query<(Entity, &mut Task<Option<RenderChunk>>)>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
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
                        println!("Spawning meshers");
                        region.should_rechunk = false;
                        // Spawn chunk-making tasks
                        for z in 0..CHUNK_DEPTH {
                            for y in 0..CHUNK_HEIGHT {
                                for x in 0..CHUNK_WIDTH {
                                    let loc = region.location;
                                    let cloc = ChunkLocation {
                                        x: x * CHUNK_SIZE,
                                        y: y * CHUNK_SIZE,
                                        z: z * CHUNK_SIZE,
                                    };
                                    let task = task_master.spawn(async move {
                                        build_render_chunk(
                                            loc,
                                            cloc,
                                        )
                                    });
                                    commands.spawn().insert(task);
                                    //println!("Spawned renderer for {},{},{}", x, y, z);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Handle created regions
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

    // Handle chunk mesh creation
    let mut n_spawned = 0;
    for (chunk_entity, mut task) in chunk_loaders.iter_mut() {
        if let Some(maybe_chunk) = future::block_on(future::poll_once(&mut *task)) {
            if let Some(mut chunk) = maybe_chunk {
                // Geometry
                for layer in chunk.layers.drain(0..) {
                    let (mx, my, mz) = layer.to_world();
                    if layer.meshes.is_some() {
                        let mut meshes = layer.meshes.unwrap();
                        for (material_id, mesh) in meshes.drain(0..) {
                            let mesh_handle = mesh_assets.add(mesh);

                            commands.spawn_bundle(PbrBundle {
                                mesh: mesh_handle,
                                material: PLANET_STORE
                                    .read()
                                    .world_material_handle
                                    .as_ref()
                                    .unwrap()[material_id]
                                    .clone(),
                                transform: Transform::from_xyz(mx, my, mz),
                                visible: Visible{is_visible: true, is_transparent: false},
                                ..Default::default()
                            });
                            n_spawned += 1;
                            //println!("Spawned mesh for {},{},{}", layer.location.x, layer.location.y, layer.location.z);
                        }
                    }
                }
            }
            commands.entity(chunk_entity).despawn();
        }
    }
    if n_spawned > 0 {
        println!("Spawned {} meshes", n_spawned);
    }
}
