use crate::prelude::*;
use legion::prelude::*;

pub fn spawn_building(ecs: &mut World, tag: &str, tile_idx: usize, region_idx: usize, complete: bool) -> usize {
    use nox_raws::*;
    let mut result = 0;
    let rlock = RAWS.read();
    if let Some(building_def) = rlock.buildings.building_by_tag(tag) {
        println!("Spawning [{}]", tag);
        let dims = if let Some(dims) = building_def.dimensions {
            dims
        } else {
            (1, 1, 1)
        };

        let identity = IdentityTag::new();
        result = identity.0;

        let entity = ecs.insert(
            (Building { complete }, Tag(tag.to_string()), identity),
            vec![(
                Name {
                    name: building_def.name.clone(),
                },
                crate::VoxelModel {
                    index: rlock.vox.get_model_idx(&building_def.vox),
                    rotation_radians: 0.0,
                },
                Description {
                    desc: building_def.description.clone(),
                },
                Position::with_tile_idx(tile_idx, region_idx, dims),
                Tint {
                    color: (1.0, 1.0, 1.0),
                },
            )],
        )[0]
        .clone();

        for provides in building_def.provides.iter() {
            if let BuildingProvides::Light { radius, color } = provides {
                ecs.add_component(
                    entity,
                    Light {
                        color: *color,
                        radius: *radius,
                        enabled: complete
                    },
                )
                .expect("Unable to add light");
                ecs.add_component(entity, FieldOfView::new(*radius))
                    .expect("Unable to add field-of-view");
            }

            if let BuildingProvides::Storage = provides {
                //println!("Added storage capacity");
                ecs.add_component(entity, Storage {})
                    .expect("Unable to add storage");
            }
        }

        println!("Added building data: {}", tag);
    } else {
        println!("Failed to spawn building: {}", tag);
    }

    result
}
