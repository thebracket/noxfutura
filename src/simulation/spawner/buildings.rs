use crate::components::{Description, Position};
use crate::raws::{BuildingProvides, RAWS};
use crate::simulation::terrain::{PlanetLocation, PLANET_STORE};
use bevy::math::Vec3;
use bevy::prelude::*;

pub fn spawn_building(
    commands: &mut Commands,
    region_id: PlanetLocation,
    x: usize,
    y: usize,
    z: usize,
    tag: &str,
) {
    let raw_lock = RAWS.read();
    if let Some(building_def) = raw_lock.buildings.building_by_tag(tag) {
        println!("Spawning [{}]: {}", tag, building_def.name);

        // Determine dimensions
        let dims = if let Some(dims) = building_def.dimensions {
            dims
        } else {
            (1, 1, 1)
        };

        // Actually perform the spawning
        let pos = Position::new(region_id, x, y, z);
        let world_pos = pos.to_world();
        let plock = PLANET_STORE.read();
        let mesh_id = crate::raws::RAWS.read().vox.get_model_idx(tag);
        let mut transform = Transform::default();
        transform.translation = world_pos;
        transform.scale = Vec3::new(0.03125, 0.03125, 0.03125);
        let building_entity = commands
            .spawn_bundle(PbrBundle {
                mesh: plock.vox_meshes[mesh_id].clone(),
                material: plock.vox_mat.as_ref().unwrap().clone(),
                transform: transform.clone(),
                visible: Visible {
                    is_visible: true,
                    is_transparent: false,
                },
                ..Default::default()
            })
            .insert(pos)
            .insert(Name::new(building_def.name.clone()))
            .insert(Description::new(building_def.description.clone()))
            .id();
        for provides in building_def.provides.iter() {
            if let BuildingProvides::Light { radius, color } = provides {
                println!("Spawning light");
                commands.entity(building_entity).insert_bundle(LightBundle {
                    transform: transform,
                    light: Light {
                        color: Color::rgb(color.0, color.1, color.2),
                        fov: 360.0,
                        depth: -(*radius as f32) .. *radius as f32,
                        range: *radius as f32,
                        intensity: 50.0,
                    },
                    ..Default::default()
                });
            }
        }
    } else {
        panic!("Unable to spawn {}", tag);
    }
}
