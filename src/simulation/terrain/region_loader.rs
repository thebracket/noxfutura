use super::{
    process_terrain_changes,
    region_chunks::{build_render_chunk, RenderChunk},
    terrain_changes_requested, ChunkLocation, PlanetLocation, Region, RegionStatus, REGIONS,
};
use crate::simulation::terrain::PLANET_STORE;
use crate::simulation::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;

pub struct MapRenderLayer {
    pub chunk_base: ChunkLocation,
    pub region_id: PlanetLocation,
    pub world_z: usize,
    pub mesh_handle: Handle<Mesh>,
}

pub fn load_regions(
    mut commands: Commands,
    task_master: Res<AsyncComputeTaskPool>,
    mut region_loaders: Query<(Entity, &mut Task<Region>)>,
    mut chunk_loaders: Query<(Entity, &mut Task<RenderChunk>)>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut chunk_query: Query<(Entity, &MapRenderLayer)>,
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
                                    let task = task_master
                                        .spawn(async move { build_render_chunk(loc, cloc) });
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
        if let Some(mut chunk) = future::block_on(future::poll_once(&mut *task)) {
            // TODO: Remove existing assets here
            chunk_query
                .iter()
                .filter(|(_, cm)| cm.region_id == chunk.region && cm.chunk_base == chunk.location)
                .for_each(|(entity, cm)| {
                    mesh_assets.remove(cm.mesh_handle.clone());
                    commands.entity(entity).despawn();
                });

            if let Some(layers) = chunk.layers.as_mut() {
                // Geometry
                for layer in layers.drain(0..) {
                    let (mx, my, mz) = layer.to_world();
                    if layer.meshes.is_some() {
                        let mut meshes = layer.meshes.unwrap();
                        for (material_id, mesh) in meshes.drain(0..) {
                            let mesh_handle = mesh_assets.add(mesh);
                            let material_handle =
                                PLANET_STORE.read().world_material_handle.as_ref().unwrap()
                                    [material_id]
                                    .clone();

                            commands
                                .spawn_bundle(PbrBundle {
                                    mesh: mesh_handle.clone(),
                                    material: material_handle.clone(),
                                    transform: Transform::from_xyz(mx, my, mz),
                                    visible: Visible {
                                        is_visible: true,
                                        is_transparent: false,
                                    },
                                    ..Default::default()
                                })
                                .insert(MapRenderLayer {
                                    chunk_base: chunk.location,
                                    world_z: layer.location.z,
                                    mesh_handle: mesh_handle.clone(),
                                    region_id: chunk.region,
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

    // Handle world changes
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
