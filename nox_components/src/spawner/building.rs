use crate::*;
use legion::*;
use nox_raws::*;

pub fn spawn_building(
    ecs: &mut World,
    tag: &str,
    tile_idx: usize,
    region_idx: usize,
    complete: bool,
    required_components: &[usize],
) -> usize {
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
            crate::VoxelModel {
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

        if !required_components.is_empty() {
            let mut ri = Vec::new();
            ri.extend_from_slice(required_components);
            let bp = Blueprint {
                required_items: ri,
                ready_to_build: false,
            };
            ecs.entry(entity).unwrap().add_component(bp);
        }

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

        let mut is_workshop = false;
        let mut has_autojobs = false;
        for reaction in rlock.reactions.reactions.iter() {
            if reaction.workshop == tag {
                is_workshop = true;
                if reaction.automatic {
                    has_autojobs = true;
                }
            }
        }
        if is_workshop {
            println!("It's a workshop - reactions are provided.");
            ecs.entry(entity).unwrap().add_component(Workshop {
                has_automatic_jobs: has_autojobs,
            });
        }

        println!("Added building data: {}", tag);
    } else {
        println!("Failed to spawn building: {}", tag);
    }

    result
}
