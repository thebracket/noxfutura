use crate::prelude::*;
use legion::prelude::*;
use nox_raws::*;

pub fn spawn_plant(ecs: &mut World, tag: &str, x: usize, y: usize, z: usize) {
    let rlock = RAWS.read();
    if let Some(plant) = rlock.plants.plant_by_tag(tag) {
        ecs.insert(
            (Vegetation {},),
            vec![(
                Identity::new(),
                Name {
                    name: plant.name.clone()
                },
                Dimensions{ width: 1, height: 1 },
                crate::VoxelModel {
                    index: rlock.vox.get_model_idx(&plant.vox),
                    rotation_radians: 0.0
                },
                Description {
                    desc: plant.description.clone(),
                },
                Position { x, y, z },
                Tint {
                    color: (1.0, 1.0, 1.0),
                },
            )],
        );
    } else {
        println!("Cannot find plant to spawn: {}", tag);
    }
}

pub fn spawn_tree(ecs: &mut World, x: usize, y: usize, z: usize) {
    let rlock = RAWS.read();
    ecs.insert(
        (Vegetation {},),
        vec![(
            Identity::new(),
            Name {
                name: "Tree".to_string()
            },
            Dimensions{ width: 3, height: 3 },
            crate::VoxelModel {
                index: rlock.vox.get_model_idx("tree"),
                rotation_radians: 0.0
            },
            Description {
                desc: "A tree".to_string(),
            },
            Position { x, y, z },
            Tint {
                color: (1.0, 1.0, 1.0),
            },
        )],
    );
}
