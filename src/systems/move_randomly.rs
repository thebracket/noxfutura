use legion::prelude::*;
use nox_components::*;
use crate::systems::RNG;
use crate::systems::REGION;
use nox_planet::{Region, mapidx};

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("move_randomly")
        .with_query(<(Write<Position>, Read<MyTurn>)>::query())
        .build(|_, ecs, _, actors| {
            actors.iter_mut(ecs)
            .filter(|(_, turn)| turn.active && turn.order == WorkOrder::MoveRandomly)
            .for_each(|(mut pos, _)| {
                let original_position = pos.clone();
                let roll = RNG.lock().range(1, 6);
                let idx = mapidx(pos.x, pos.y, pos.z);

                let destination = match roll {
                    1 => if REGION.read().flag(idx, Region::CAN_GO_NORTH) {
                        Position{ x: original_position.x, y: original_position.y - 1, z:original_position.z }
                    } else {
                        original_position
                    },

                    2 => if REGION.read().flag(idx, Region::CAN_GO_SOUTH) {
                        Position{ x: original_position.x, y: original_position.y + 1, z:original_position.z }
                    } else {
                        original_position
                    },

                    3 => if REGION.read().flag(idx, Region::CAN_GO_EAST) {
                        Position{ x: original_position.x + 1, y: original_position.y, z:original_position.z }
                    } else {
                        original_position
                    },

                    4 => if REGION.read().flag(idx, Region::CAN_GO_WEST) {
                        Position{ x: original_position.x - 1, y: original_position.y, z:original_position.z }
                    } else {
                        original_position
                    },

                    5 => if REGION.read().flag(idx, Region::CAN_GO_UP) {
                        println!("UP");
                        Position{ x: original_position.x, y: original_position.y, z:original_position.z - 1 }
                    } else {
                        original_position
                    },

                    _ => if REGION.read().flag(idx, Region::CAN_GO_DOWN) {
                        println!("DOWN");
                        Position{ x: original_position.x, y: original_position.y, z:original_position.z + 1 }
                    } else {
                        original_position
                    }
                };

                /*pos.x = destination.x;
                pos.y = destination.y;
                pos.z = destination.z;*/ // Renable me
                crate::messaging::vox_moved();
            });
        }
    )
}
