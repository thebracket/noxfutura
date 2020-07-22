use crate::prelude::*;
use legion::prelude::*;
use nox_raws::*;

fn spawn_item_common(ecs: &mut World, tag: &str) -> Option<(Entity, usize)> {
    let raws = RAWS.read();
    if let Some(item) = raws.items.item_by_tag(tag) {
        println!("Spawning item [{}]", tag);
        let id = IdentityTag::new();
        let new_identity = id.0;
        let entity = ecs.insert(
            (Item {}, Tag(tag.to_string()), id),
            vec![(
                Name {
                    name: item.name.clone(),
                },
                Description {
                    desc: item.description.clone(),
                },
                crate::VoxelModel {
                    index: raws.vox.get_model_idx(&item.vox),
                    rotation_radians: 0.0,
                },
                Tint {
                    color: (1.0, 1.0, 1.0),
                },
            )],
        )[0]
        .clone();

        for it in item.item_type.iter() {
            match it {
                ItemDefType::ToolChopping => ecs
                    .add_component(
                        entity,
                        Tool {
                            usage: ToolType::Chopping,
                        },
                    )
                    .expect("Fail to spawn component"),
                ItemDefType::ToolDigging => ecs
                    .add_component(
                        entity,
                        Tool {
                            usage: ToolType::Digging,
                        },
                    )
                    .expect("Fail to spawn component"),
                ItemDefType::ToolFarming => ecs
                    .add_component(
                        entity,
                        Tool {
                            usage: ToolType::Farming,
                        },
                    )
                    .expect("Fail to spawn component"),
                _ => {}
            }
        }

        Some((entity, new_identity))
    } else {
        println!("Warning: don't know how to spawn item {}", tag);
        None
    }
}

pub fn spawn_item_on_ground(
    ecs: &mut World,
    tag: &str,
    x: usize,
    y: usize,
    z: usize,
    region_idx: usize,
) -> Option<usize> {
    if let Some((entity, id)) = spawn_item_common(ecs, tag) {
        ecs.add_component(entity, Position::with_tile(x, y, z, region_idx, (1, 1, 1)))
            .expect("Failed to add component");
        Some(id)
    } else {
        None
    }
}

pub fn spawn_item_in_container(ecs: &mut World, tag: &str, container: usize) -> Option<usize> {
    if let Some((entity, id)) = spawn_item_common(ecs, tag) {
        ecs.add_component(entity, Position::stored(container))
            .expect("Failed to add component");
        Some(id)
    } else {
        None
    }
}

pub fn spawn_item_worn(ecs: &mut World, tag: &str, wearer: usize) -> Option<usize> {
    if let Some((entity, id)) = spawn_item_common(ecs, tag) {
        ecs.add_component(entity, Position::worn(wearer))
            .expect("Failed to add component");
        Some(id)
    } else {
        None
    }
}

pub fn spawn_item_carried(ecs: &mut World, tag: &str, wearer: usize) -> Option<usize> {
    if let Some((entity, id)) = spawn_item_common(ecs, tag) {
        ecs.add_component(entity, Position::carried(wearer))
            .expect("Failed to add component");
        Some(id)
    } else {
        None
    }
}
