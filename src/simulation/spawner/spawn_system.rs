use super::{SpawnRequest, SPAWNS};
use crate::{
    components::Position,
    simulation::{
        idxmap,
        terrain::{PlanetLocation, PLANET_STORE},
    },
};
use bevy::prelude::*;

pub fn spawn_game_entities(mut commands: Commands) {
    if SPAWNS.read().is_empty() {
        return;
    }

    let mut spawns = SPAWNS.write();
    spawns.iter().for_each(|s| {
        let (x, y, z) = idxmap(s.tile_location);
        let (rx, ry, rz) = s.region_id.to_world();
        let mx = rx + x as f32;
        let my = ry + y as f32;
        let mz = rz + z as f32;

        let mut transform = Transform::default();
        transform.translation = Vec3::new(mx, my, mz);
        transform.scale = Vec3::new(0.005, 0.005, 0.005);
        transform.rotate(Quat::from_rotation_ypr(0.0, 1.5708, 0.0));

        match &s.event {
            SpawnRequest::Tree => {
                let (x, y, z) = idxmap(s.tile_location);
                let pos = Position::new(s.region_id, x, y, z);
                let plock = PLANET_STORE.read();
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: plock.tree_handle.as_ref().unwrap().clone(),
                        material: plock.tree_mat.as_ref().unwrap().clone(),
                        transform,
                        visible: Visible {
                            is_visible: true,
                            is_transparent: false,
                        },
                        ..Default::default()
                    })
                    .insert(pos);
            }
            SpawnRequest::RawEntity { tag } => {
                let (x, y, z) = idxmap(s.tile_location);
                spawn_vox_mesh(&mut commands, s.region_id, x, y, z, &tag);
            }
        }
    });
    spawns.clear();
}

fn spawn_vox_mesh(
    commands: &mut Commands,
    region_id: PlanetLocation,
    x: usize,
    y: usize,
    z: usize,
    tag: &str,
) {
    let pos = Position::new(region_id, x, y, z);
    let world_pos = pos.to_world();
    let plock = PLANET_STORE.read();
    let mesh_id = crate::raws::RAWS.read().vox.get_model_idx(tag);
    let mut transform = Transform::default();
    transform.translation = world_pos;
    transform.scale = Vec3::new(0.03125, 0.03125, 0.03125);
    commands
        .spawn_bundle(PbrBundle {
            mesh: plock.vox_meshes[mesh_id].clone(),
            material: plock.vox_mat.as_ref().unwrap().clone(),
            transform,
            visible: Visible {
                is_visible: true,
                is_transparent: false,
            },
            ..Default::default()
        })
        .insert(pos);
}
