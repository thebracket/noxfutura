use super::*;
use legion::prelude::*;

pub fn spawn_building(ecs: &mut World, tag: &str, x:usize, y:usize, z:usize) {
    let rlock = crate::raws::RAWS.read();
    ecs.insert(
        (Building {},),
        (0..1).map(|_| {
            (
                Position{x, y, z},
                Dimensions{width: 1, height: 1},
                VoxelModel{index: rlock.vox.get_model_idx(tag) }
            )
        }),
    );
    println!("Added building data");
}