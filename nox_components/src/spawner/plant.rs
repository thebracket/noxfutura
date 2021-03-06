use crate::*;
use legion::*;
use nox_raws::*;

pub fn spawn_plant(
    ecs: &mut World,
    tag: &str,
    x: usize,
    y: usize,
    z: usize,
    region_idx: usize,
    size: f32,
) {
    let rlock = RAWS.read();
    if let Some(plant) = rlock.plants.plant_by_tag(tag) {
        ecs.push((
            Vegetation { size },
            IdentityTag::new(),
            Name {
                name: plant.name.clone(),
            },
            /*nox_components::ObjModel{
                index: 7,
                rotation_radians: 0.0
            },*/
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

pub fn spawn_tree(
    ecs: &mut World,
    x: usize,
    y: usize,
    z: usize,
    region_idx: usize,
    model_id: usize,
    size: f32,
) {
    //let rlock = RAWS.read();
    ecs.push((
        Tree { chop: false },
        IdentityTag::new(),
        Name {
            name: "Tree".to_string(),
        },
        crate::ObjModel {
            index: model_id,
            rotation_radians: 0.0,
            scale: size,
        },
        Description {
            desc: "A tree".to_string(),
        },
        Position::with_tile(x, y, z, region_idx, (3, 3, 3)),
        Tint { color: 0 },
        Health::new(10),
    ));
}
