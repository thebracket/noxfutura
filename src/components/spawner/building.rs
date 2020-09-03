use crate::components::*;
use legion::*;

pub fn spawn_building(
    ecs: &mut World,
    tag: &str,
    tile_idx: usize,
    region_idx: usize,
    complete: bool,
) -> usize {
    use crate::raws::*;
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

        let entity = ecs.push((
            identity,
            Building { complete },
            Tag(tag.to_string()),
            Name {
                name: building_def.name.clone(),
            },
            crate::components::VoxelModel {
                index: rlock.vox.get_model_idx(&building_def.vox),
                rotation_radians: 0.0,
            },
            Description {
                desc: building_def.description.clone(),
            },
            Position::with_tile_idx(tile_idx, region_idx, dims),
            Tint { color: 0 },
        ));
        println!("New building ID: {}", identity.0);

        for provides in building_def.provides.iter() {
            if let BuildingProvides::Light { radius, color } = provides {
                ecs.entry(entity).unwrap().add_component(Light {
                    color: *color,
                    radius: *radius,
                    enabled: complete,
                });
                ecs.entry(entity)
                    .unwrap()
                    .add_component(FieldOfView::new(*radius));
            }

            if let BuildingProvides::Storage = provides {
                //println!("Added storage capacity");
                ecs.entry(entity).unwrap().add_component(Storage {});
            }
        }

        println!("Added building data: {}", tag);
    } else {
        println!("Failed to spawn building: {}", tag);
    }

    result
}
