use super::super::messaging;
use nox_components::*;
use crate::modes::playgame::systems::REGION;
use nox_planet::pathfinding::a_star_search;
use legion::*;
use nox_planet::MiningMap;
use nox_spatial::idxmap;
use bengine::geometry::Point3;

#[system(for_each)]
pub fn mining(turn: &MyTurn, pos: &Position, id: &IdentityTag, #[resource] mining: &MiningMap) {
    if turn.active
        && turn.shift == ScheduleTime::Work
        && match turn.job {
            JobType::Mining { .. } => true,
            _ => false,
        }
    {
        if let JobType::Mining {
            step,
            tool_id
        } = &turn.job
        {
            println!("Loc at step: {:?}", pos);
            match step {
                MiningSteps::FindPick => {
                    println!("Step: FindPick");
                    let mut rlock = REGION.write();
                    let maybe_tool_pos = rlock
                        .jobs_board
                        .find_and_claim_tool(ToolType::Digging, id.0);
                    if let Some((tool_id, tool_pos)) = maybe_tool_pos {
                        let path = a_star_search(pos.get_idx(), tool_pos, &*rlock);
                        std::mem::drop(rlock);
                        if !path.success {
                            println!("No path to tool available - abandoning mining");
                            messaging::cancel_job(id.0);
                            messaging::relinquish_claim(tool_id, tool_pos);
                        } else {
                            messaging::job_changed(
                                id.0,
                                JobType::Mining {
                                    step: MiningSteps::TravelToPick { path: path.steps },
                                    tool_id: Some(tool_id),
                                },
                            );
                        }
                    } else {
                        println!("No tool available - abandoning mining");
                        messaging::cancel_job(id.0);
                    }
                }
                MiningSteps::TravelToPick { path } => {
                    println!("Step: TravelToPick");
                    if path.len() > 1 {
                        messaging::follow_job_path(id.0);
                    } else {
                        messaging::job_changed(
                            id.0,
                            JobType::Mining {
                                step: MiningSteps::CollectPick,
                                tool_id: *tool_id,
                            },
                        );
                    }
                }
                MiningSteps::CollectPick => {
                    println!("Step: CollectPick");
                    messaging::equip_tool(id.0, tool_id.unwrap());
                    messaging::job_changed(
                        id.0,
                        JobType::Mining {
                            step: MiningSteps::TravelToMine {},
                            tool_id: *tool_id,
                        },
                    );
                }
                MiningSteps::TravelToMine => {
                    println!("Step: Travel to mine");
                    let idx = pos.get_idx();
                    let distance = mining.dijkstra[idx];
                    if distance < f32::MAX {
                        println!("Distance to mine target: {}", distance);
                        if distance < 1.0 {
                            messaging::job_changed(
                                id.0,
                                JobType::Mining {
                                    step: MiningSteps::Dig {},
                                    tool_id: *tool_id,
                                },
                            );
                        } else {
                            // Walk the Dijkstra
                            let rlock = REGION.read();
                            if let Some(exit) = mining.find_lowest_exit(idx, &rlock) {
                                let (nx, ny, nz) = idxmap(exit);
                                messaging::entity_moved(id.0, &Point3::new(nx, ny, nz));
                            } else {
                                // Abort!
                                messaging::drop_item(tool_id.unwrap(), pos.get_idx());
                                messaging::relinquish_claim(tool_id.unwrap(), pos.get_idx());
                                messaging::conclude_job(id.0);
                            }
                        }
                    } else {
                        // Abort!
                        println!("Nothing to mine");
                        messaging::drop_item(tool_id.unwrap(), pos.get_idx());
                        messaging::relinquish_claim(tool_id.unwrap(), pos.get_idx());
                        messaging::conclude_job(id.0);
                    }
                }
                MiningSteps::Dig => {
                    println!("Step: Dig");
                    messaging::dig_at(id.0, pos.get_idx());
                    messaging::conclude_job(id.0);
                    messaging::drop_item(tool_id.unwrap(), pos.get_idx());
                    messaging::relinquish_claim(tool_id.unwrap(), pos.get_idx());
                }
            }
        }
    }
}