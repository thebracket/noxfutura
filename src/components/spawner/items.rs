use legion::prelude::*;
use crate::raws::*;
use crate::components::*;

pub fn spawn_item_on_ground(ecs: &mut World, tag: &str, x: usize, y: usize, z:usize) {
    let raws = RAWS.read();
    if let Some(item) = raws.items.item_by_tag(tag) {
        ecs.insert(
            (Item {},),
            vec![(
                Identity::new(),
                Position { x, y, z },
                Name { name: item.name.clone() },
                Description {
                    desc: item.description.clone(),
                },
                crate::components::VoxelModel { index: raws.vox.get_model_idx( &item.vox ) },
                Tint { color: (1.0, 1.0, 1.0) },
                Dimensions{ width: 1, height: 1},
            )],
        );
        println!("Ground spawned a {}", tag);
    } else {
        println!("Warning: Don't know how to spawn item [{}]", tag);
    }
}

pub fn spawn_item_in_container(ecs: &mut World, tag: &str, container: usize) {
    let raws = RAWS.read();
    if let Some(item) = raws.items.item_by_tag(tag) {
        ecs.insert(
            (Item {},),
            vec![(
                Identity::new(),
                ItemStored{ container },
                Name { name: item.name.clone() },
                Description {
                    desc: item.description.clone(),
                },
                crate::components::VoxelModel { index: raws.vox.get_model_idx( &item.vox ) },
                Tint { color: (1.0, 1.0, 1.0) },
                Dimensions{ width: 1, height: 1},
            )],
        );
    } else {
        println!("Warning: Don't know how to spawn item [{}]", tag);
    }
}

pub fn spawn_item_worn(ecs: &mut World, tag: &str, wearer: usize) {
    let raws = RAWS.read();
    if let Some(item) = raws.items.item_by_tag(tag) {
        ecs.insert(
            (Item {},),
            vec![(
                Identity::new(),
                ItemWorn{ wearer },
                Name { name: item.name.clone() },
                Description {
                    desc: item.description.clone(),
                },
                crate::components::VoxelModel { index: raws.vox.get_model_idx( &item.vox ) },
                Tint { color: (1.0, 1.0, 1.0) },
                Dimensions{ width: 1, height: 1},
            )],
        );
    } else {
        println!("Warning: Don't know how to spawn item [{}]", tag);
    }
}

pub fn spawn_item_carried(ecs: &mut World, tag: &str, wearer: usize) {
    let raws = RAWS.read();
    if let Some(item) = raws.items.item_by_tag(tag) {
        ecs.insert(
            (Item {},),
            vec![(
                Identity::new(),
                ItemCarried{ wearer },
                Name { name: item.name.clone() },
                Description {
                    desc: item.description.clone(),
                },
                crate::components::VoxelModel { index: raws.vox.get_model_idx( &item.vox ) },
                Tint { color: (1.0, 1.0, 1.0) },
            )],
        );
    } else {
        println!("Warning: Don't know how to spawn item [{}]", tag);
    }
}