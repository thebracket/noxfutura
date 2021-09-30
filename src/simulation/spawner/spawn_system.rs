use super::{buildings::spawn_building, SpawnRequest, SPAWNS};
use crate::{
    components::Position,
    simulation::terrain::PLANET_STORE,
};
use bevy::prelude::*;

pub fn spawn_game_entities(mut commands: Commands) {
    if SPAWNS.read().is_empty() {
        return;
    }

    let mut spawns = SPAWNS.write();
    spawns.iter().for_each(|s| {
        let world_position = s.position.to_world();

        let mut transform = Transform::default();
        transform.translation = world_position;
        transform.scale = Vec3::new(0.005, 0.005, 0.005);
        transform.rotate(Quat::from_rotation_ypr(0.0, 1.5708, 0.0));

        match &s.event {
            SpawnRequest::Tree => {
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
                    .insert(s.position);
            }
            SpawnRequest::RawEntity { tag } => {
                if tag == "stairs_up" || tag == "stairs_down" || tag == "stairs_updown" {
                    spawn_vox_mesh(
                        &mut commands,
                        s.position,
                        &tag,
                    );
                } else {
                    spawn_building(
                        &mut commands,
                        s.position,
                        &tag,
                    );
                }
            }
        }
    });
    spawns.clear();
}

fn spawn_vox_mesh(commands: &mut Commands, position: Position, tag: &str) {
    let world_pos = position.to_world();
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
        .insert(position);
}
