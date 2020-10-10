use crate::*;
use nox_raws::*;
use legion::*;

fn spawn_item_common(ecs: &mut World, tag: &str, material: usize) -> Option<(Entity, usize)> {
    let raws = RAWS.read();
    if let Some(item) = raws.items.item_by_tag(tag) {
        println!("Spawning item [{}]", tag);
        let id = IdentityTag::new();
        let new_identity = id.0;
        let entity = ecs
            .push((
                Item {},
                Tag(tag.to_string()),
                id,
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
                Tint { color: 0 },
                Material(material),
            ))
            .clone();

        for it in item.item_type.iter() {
            match it {
                ItemDefType::ToolChopping => ecs.entry(entity).unwrap().add_component(Tool {
                    usage: ToolType::Chopping,
                }),
                ItemDefType::ToolDigging => ecs.entry(entity).unwrap().add_component(Tool {
                    usage: ToolType::Digging,
                }),
                ItemDefType::ToolFarming => ecs.entry(entity).unwrap().add_component(Tool {
                    usage: ToolType::Farming,
                }),
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
    material: usize,
) -> Option<usize> {
    if let Some((entity, id)) = spawn_item_common(ecs, tag, material) {
        ecs.entry(entity)
            .unwrap()
            .add_component(Position::with_tile(x, y, z, region_idx, (1, 1, 1)));
        Some(id)
    } else {
        None
    }
}

pub fn spawn_item_in_container(
    ecs: &mut World,
    tag: &str,
    container: usize,
    material: usize,
) -> Option<usize> {
    if let Some((entity, id)) = spawn_item_common(ecs, tag, material) {
        ecs.entry(entity)
            .unwrap()
            .add_component(Position::stored(container));
        Some(id)
    } else {
        None
    }
}

pub fn spawn_item_worn(
    ecs: &mut World,
    tag: &str,
    wearer: usize,
    material: usize,
) -> Option<usize> {
    if let Some((entity, id)) = spawn_item_common(ecs, tag, material) {
        ecs.entry(entity)
            .unwrap()
            .add_component(Position::worn(wearer));
        Some(id)
    } else {
        None
    }
}

pub fn spawn_item_carried(
    ecs: &mut World,
    tag: &str,
    wearer: usize,
    material: usize,
) -> Option<usize> {
    if let Some((entity, id)) = spawn_item_common(ecs, tag, material) {
        ecs.entry(entity)
            .unwrap()
            .add_component(Position::carried(wearer));
        Some(id)
    } else {
        None
    }
}
