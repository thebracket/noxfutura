use crate::components::*;
use crate::raws::*;
use legion::*;

pub fn spawn_plant(ecs: &mut World, tag: &str, x: usize, y: usize, z: usize, region_idx: usize) {
    let rlock = RAWS.read();
    if let Some(plant) = rlock.plants.plant_by_tag(tag) {
        ecs.push((
            Vegetation {},
            IdentityTag::new(),
            Name {
                name: plant.name.clone(),
            },
            crate::components::VoxelModel {
                index: rlock.vox.get_model_idx(&plant.vox),
                rotation_radians: 0.0,
            },
            Description {
                desc: plant.description.clone(),
            },
            Position::with_tile(x, y, z, region_idx, (1, 1, 1)),
            Tint { color: 0 },
        ));
    } else {
        println!("Cannot find plant to spawn: {}", tag);
    }
}

pub fn spawn_tree(ecs: &mut World, x: usize, y: usize, z: usize, region_idx: usize) {
    //let rlock = RAWS.read();
    ecs.push((
        Tree {},
        IdentityTag::new(),
        Name {
            name: "Tree".to_string(),
        },
        crate::components::ObjModel {
            index: 0,
            rotation_radians: 0.0,
        },
        Description {
            desc: "A tree".to_string(),
        },
        Position::with_tile(x, y, z, region_idx, (3, 3, 3)),
        Tint { color: 0 },
    ));
}
