use super::*;
use legion::prelude::*;

pub fn spawn_building(ecs: &mut World, tag: &str, x:usize, y:usize, z:usize) {
    let rlock = crate::raws::RAWS.read();
    if let Some(building_def) = rlock.buildings.building_by_tag(tag) {

        let dims = if let Some(dims) = building_def.dimensions {
            Dimensions{width: dims.0 as i32, height: dims.1 as i32}
        } else {
            Dimensions{width: 1, height: 1}
        };

        let entity = ecs.insert(
            (Building{},),
            vec![
                (
                    Name{name: building_def.name.clone()},
                    dims,
                    VoxelModel{index: rlock.vox.get_model_idx(&building_def.vox)},
                    Description{desc: building_def.description.clone()},
                    Position{x, y, z}
                )
            ]
        );

        println!("Added building data: {}", tag);
    } else {
        println!("Failed to spawn building: {}", tag);
    }
}