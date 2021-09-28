use super::{SPAWNS, SpawnRequest};
use crate::simulation::{idxmap, terrain::PLANET_STORE};
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
                let plock = PLANET_STORE.read();
                commands.spawn_bundle(PbrBundle {
                    mesh: plock.tree_handle.as_ref().unwrap().clone(),
                    material: plock.tree_mat.as_ref().unwrap().clone(),
                    transform,
                    visible: Visible {
                        is_visible: true,
                        is_transparent: false,
                    },
                    ..Default::default()
                });
            }
            SpawnRequest::RawEntity{tag} => {
                spawn_vox_mesh(&mut commands, mx, my, mz, &tag);
            }
        }
    });
    spawns.clear();
}

fn spawn_vox_mesh(commands: &mut Commands, mx:f32, my: f32, mz: f32, tag: &str) {
    let plock = PLANET_STORE.read();
    let mesh_id = crate::raws::RAWS.read().vox.get_model_idx(tag);
    let mut transform = Transform::default();
    transform.translation = Vec3::new(mx, my, mz);
    transform.scale = Vec3::new(0.03125, 0.03125, 0.03125);
    commands.spawn_bundle(PbrBundle {
        mesh: plock.vox_meshes[mesh_id].clone(),
        material: plock.vox_mat.as_ref().unwrap().clone(),
        transform,
        visible: Visible {
            is_visible: true,
            is_transparent: false,
        },
        ..Default::default()
    });
}