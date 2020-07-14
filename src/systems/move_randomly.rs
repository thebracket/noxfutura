use crate::systems::REGION;
use crate::systems::RNG;
use legion::prelude::*;
use nox_components::*;
use nox_planet::Region;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("move_randomly")
        .with_query(<(Write<Position>, Read<MyTurn>, Read<Identity>)>::query())
        .build(|_, ecs, _, actors| {
            actors
                .iter_mut(ecs)
                .filter(|(_, turn, _)| turn.active && turn.order == WorkOrder::MoveRandomly)
                .for_each(|(pos, _, id)| {
                    let roll = RNG.lock().range(1, 6);
                    let idx = pos.get_idx();

                    let delta = match roll {
                        1 => {
                            if REGION.read().flag(idx, Region::CAN_GO_NORTH) {
                                (0, -1, 0)
                            } else {
                                (0, 0, 0)
                            }
                        }

                        2 => {
                            if REGION.read().flag(idx, Region::CAN_GO_SOUTH) {
                                (0, 1, 0)
                            } else {
                                (0, 0, 0)
                            }
                        }

                        3 => {
                            if REGION.read().flag(idx, Region::CAN_GO_EAST) {
                                (1, 0, 0)
                            } else {
                                (0, 0, 0)
                            }
                        }

                        4 => {
                            if REGION.read().flag(idx, Region::CAN_GO_WEST) {
                                (-1, 0, 0)
                            } else {
                                (0, 0, 0)
                            }
                        }

                        5 => {
                            if REGION.read().flag(idx, Region::CAN_GO_UP) {
                                (0, 0, 1)
                            } else {
                                (0, 0, 0)
                            }
                        }

                        _ => {
                            if REGION.read().flag(idx, Region::CAN_GO_DOWN) {
                                (0, 0, -1)
                            } else {
                                (0, 0, 0)
                            }
                        }
                    };

                    let mut destination = pos.as_point3();
                    destination.x += delta.0;
                    destination.y += delta.1;
                    destination.z += delta.2;
                    crate::messaging::entity_moved(id.id, &destination);
                });
        })
}