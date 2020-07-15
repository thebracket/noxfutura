use crate::messaging;
use crate::systems::REGION;
use nox_planet::pathfinding::a_star_search;
use legion::prelude::*;
use nox_components::*;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("lumberjack")
        .with_query(<(Read<MyTurn>, Read<Position>, Tagged<IdentityTag>)>::query())
        .build(|_, ecs, _, actors| {
            // Look for a job to do
            actors
                .iter_mut(ecs)
                .filter(|(turn, _, _)| {
                    // Filter out anything that isn't a lumberjack job
                    turn.active
                        && turn.shift == ScheduleTime::Work
                        && match turn.job {
                            JobType::FellTree { .. } => true,
                            _ => false,
                        }
                })
                .for_each(|(turn, pos, id)| {
                    if let JobType::FellTree {
                        tree_id,
                        tree_pos,
                        step,
                        tool_id,
                    } = &turn.job
                    {
                        match step {
                            LumberjackSteps::FindAxe => {
                                let mut rlock = REGION.write();
                                let maybe_tool_pos = rlock
                                    .jobs_board
                                    .find_and_claim_tool(ToolType::Chopping, id.0);
                                if let Some((tool_id, tool_pos)) = maybe_tool_pos {
                                    let path = a_star_search(pos.get_idx(), tool_pos, &*rlock);
                                    if !path.success {
                                        println!(
                                            "No path to tool available - abandoning lumberjacking"
                                        );
                                        messaging::cancel_job(id.0);
                                        messaging::relinquish_claim(tool_id);
                                    } else {
                                        messaging::job_changed(
                                            id.0,
                                            JobType::FellTree {
                                                tree_id: *tree_id,
                                                tree_pos: *tree_pos,
                                                step: LumberjackSteps::TravelToAxe {
                                                    path: path.steps,
                                                },
                                                tool_id: Some(tool_id),
                                            },
                                        );
                                    }
                                } else {
                                    println!("No tool available - abandoning lumberjacking");
                                    messaging::cancel_job(id.0);
                                }
                            }
                            LumberjackSteps::TravelToAxe { path } => {
                                if path.len() > 1 {
                                    crate::messaging::follow_job_path(id.0);
                                } else {
                                    messaging::job_changed(
                                        id.0,
                                        JobType::FellTree {
                                            tree_id: *tree_id,
                                            tree_pos: *tree_pos,
                                            step: LumberjackSteps::CollectAxe,
                                            tool_id: *tool_id,
                                        },
                                    );
                                }
                            }
                            LumberjackSteps::CollectAxe => {
                                messaging::equip_tool(id.0, tool_id.unwrap());
                                messaging::job_changed(
                                    id.0,
                                    JobType::FellTree {
                                        tree_id: *tree_id,
                                        tree_pos: *tree_pos,
                                        step: LumberjackSteps::FindTree {},
                                        tool_id: *tool_id,
                                    },
                                );
                            }
                            LumberjackSteps::FindTree {} => {
                                let rlock = REGION.read();
                                let path = a_star_search(pos.get_idx(), *tree_pos, &*rlock);
                                if path.success {
                                    messaging::job_changed(
                                        id.0,
                                        JobType::FellTree {
                                            tree_id: *tree_id,
                                            tree_pos: *tree_pos,
                                            step: LumberjackSteps::TravelToTree {
                                                path: path.steps,
                                            },
                                            tool_id: *tool_id,
                                        },
                                    );
                                } else {
                                    println!(
                                        "No path to tree available - abandoning lumberjacking"
                                    );
                                    messaging::cancel_job(id.0);
                                    if let Some(tool_id) = tool_id {
                                        messaging::relinquish_claim(*tool_id);
                                        messaging::drop_item(*tool_id, pos.get_idx());
                                    }
                                }
                            }
                            LumberjackSteps::TravelToTree { path } => {
                                if path.len() > 1 {
                                    messaging::follow_job_path(id.0);
                                } else {
                                    messaging::job_changed(
                                        id.0,
                                        JobType::FellTree {
                                            tree_id: *tree_id,
                                            tree_pos: *tree_pos,
                                            step: LumberjackSteps::ChopTree {},
                                            tool_id: *tool_id,
                                        },
                                    );
                                }
                            }
                            LumberjackSteps::ChopTree {} => {
                                messaging::chop_tree(id.0, *tree_id);
                                messaging::conclude_job(id.0);
                                messaging::drop_item(tool_id.unwrap(), pos.get_idx());
                                messaging::relinquish_claim(tool_id.unwrap());
                            }
                        }
                    } else {
                        panic!("Not doing a lumberjack job but wound up in the LJ system!");
                    }
                });
        })
}
