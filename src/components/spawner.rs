use super::*;
use legion::prelude::*;

pub fn spawn_building(ecs: &mut World, tag: &str, x:usize, y:usize, z:usize) {
    ecs.insert(
        (Building {},),
        (0..1).map(|_| {
            (
                Position{x: 1, y: 1, z: 1},
                Dimensions{width: 1, height: 1},
                VoxelModel{index: 0}
            )
        }),
    );
}