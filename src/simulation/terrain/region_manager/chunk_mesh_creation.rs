use bevy::{prelude::*, tasks::Task};
use futures_lite::future;
use crate::simulation::terrain::MapRenderLayer;
use crate::simulation::terrain::PLANET_STORE;
use super::RenderChunk;

/// Receives results/futures from RenderChunk creation tasks,
/// and turns them into actual usable meshes/geometry with
/// associated in-game MapRenderLayer to identify them.
pub fn chunk_mesh_creation_system(
    mut commands: Commands,
    mut chunk_loaders: Query<(Entity, &mut Task<RenderChunk>)>,
    chunk_query: Query<(Entity, &MapRenderLayer)>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
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
                        }
                    }
                }
            }
            commands.entity(chunk_entity).despawn();
        }
    }
}