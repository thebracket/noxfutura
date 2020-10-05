use super::super::messaging;
use crate::components::*;
use crate::modes::playgame::systems::REGION;
use crate::planet::pathfinding::a_star_search;
use legion::*;

#[system(for_each)]
pub fn lumberjack(turn: &MyTurn, pos: &Position, id: &IdentityTag) {
    if turn.active
        && turn.shift == ScheduleTime::Work
        && match turn.job {
            JobType::FellTree { .. } => true,
            _ => false,
        }
    {
        if let JobType::FellTree {
            tree_id,
            tree_pos,
            step,
            tool_id,
        } = &turn.job
        {
            println!("Loc at step: {:?}", pos);
            match step {
                LumberjackSteps::FindAxe => {
                    println!("Step: FindAxe");
                    let mut rlock = REGION.write();
                    let maybe_tool_pos = rlock
                        .jobs_board
                        .find_and_claim_tool(ToolType::Chopping, id.0);
                    if let Some((tool_id, tool_pos)) = maybe_tool_pos {
                        let path = a_star_search(pos.get_idx(), tool_pos, &*rlock);
                        if !path.success {
                            println!("No path to tool available - abandoning lumberjacking");
                            messaging::cancel_job(id.0);
                            messaging::relinquish_claim(tool_id, tool_pos);
                        } else {
                            messaging::job_changed(
                                id.0,
                                JobType::FellTree {
                                    tree_id: *tree_id,
                                    tree_pos: *tree_pos,
                                    step: LumberjackSteps::TravelToAxe { path: path.steps },
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
                    println!("Step: TravelToAxe");
                    if path.len() > 1 {
                        messaging::follow_job_path(id.0);
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
                    println!("Step: CollectAxe");
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
                    println!("Step: FindTree");
                    let rlock = REGION.read();
                    let path = a_star_search(pos.get_idx(), *tree_pos, &*rlock);
                    println!("{:?}", path);
                    if path.success {
                        messaging::job_changed(
                            id.0,
                            JobType::FellTree {
                                tree_id: *tree_id,
                                tree_pos: *tree_pos,
                                step: LumberjackSteps::TravelToTree { path: path.steps },
                                tool_id: *tool_id,
                            },
                        );
                    } else {
                        println!("No path to tree available - abandoning lumberjacking");
                        messaging::cancel_job(id.0);
                        if let Some(tool_id) = tool_id {
                            messaging::relinquish_claim(*tool_id, pos.get_idx());
                            messaging::drop_item(*tool_id, pos.get_idx());
                            messaging::vox_moved();
                        }
                    }
                }
                LumberjackSteps::TravelToTree { path } => {
                    println!("Step: TravelToTree");
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
                    println!("Step: ChopTree");
                    messaging::chop_tree(id.0, *tree_id);
                    messaging::conclude_job(id.0);
                    messaging::drop_item(tool_id.unwrap(), pos.get_idx());
                    messaging::relinquish_claim(tool_id.unwrap(), pos.get_idx());
                }
            }
        } else {
            panic!("Not doing a lumberjack job but wound up in the LJ system!");
        }
    }
}
