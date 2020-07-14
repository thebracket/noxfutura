use crate::systems::REGION;
use bracket_pathfinding::prelude::a_star_search;
use legion::prelude::*;
use nox_components::*;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("lumberjack")
        .with_query(<(Read<MyTurn>, Read<Position>, Read<Identity>)>::query())
        .build(|_, ecs, _, actors| {
            // Look for a job to do
            actors
                .iter_mut(ecs)
                .filter(|(turn, _, _)| {
                    // Filter out anything that isn't a lumberjack job
                    turn.active && turn.shift == ScheduleTime::Work
                    && match turn.job {
                        JobType::FellTree{..} => true,
                        _ => false
                    }
                })
                .for_each(|(turn, pos, id)| {
                    if let JobType::FellTree{ tree_id, tree_pos, step } = &turn.job {
                        match step {
                            LumberjackSteps::FindAxe => {
                                let mut rlock = REGION.write();
                                let maybe_tool_pos = rlock.jobs_board.find_and_claim_tool(ToolType::Chopping, id.id);
                                if let Some((tool_id, tool_pos)) = maybe_tool_pos {
                                    let path = a_star_search(
                                        pos.get_idx(),
                                        tool_pos,
                                        &*rlock
                                    );
                                    if !path.success {
                                        println!("No path to tool available - abandoning lumberjacking");
                                        crate::messaging::cancel_job(id.id);
                                    } else {
                                        crate::messaging::job_changed(id.id,
                                            JobType::FellTree{
                                                tree_id: *tree_id,
                                                tree_pos: *tree_pos,
                                                step: LumberjackSteps::TravelToAxe{ path: path.steps, tool_id  }
                                            }
                                        );
                                    }
                                } else {
                                    println!("No tool available - abandoning lumberjacking");
                                    crate::messaging::cancel_job(id.id);
                                }

                            }
                            LumberjackSteps::TravelToAxe{path, tool_id} => {
                                if path.len() > 1 {
                                    crate::messaging::follow_job_path(id.id);
                                } else {
                                    println!("We're adjacent to the target item now. Pretending we picked it up");
                                    crate::messaging::job_changed(id.id,
                                        JobType::FellTree{
                                            tree_id: *tree_id,
                                            tree_pos: *tree_pos,
                                            step: LumberjackSteps::CollectAxe{ tool_id: *tool_id  }
                                        }
                                    );
                                }
                            }
                            LumberjackSteps::CollectAxe{ tool_id } => {
                                crate::messaging::equip_tool(id.id, *tool_id);
                                crate::messaging::job_changed(id.id,
                                    JobType::FellTree{
                                        tree_id: *tree_id,
                                        tree_pos: *tree_pos,
                                        step: LumberjackSteps::FindTree{}
                                    }
                                );
                            }
                            LumberjackSteps::FindTree{} => {
                                println!("Tree pos: {}", tree_pos);
                                let rlock = REGION.read();
                                let path = a_star_search(
                                    pos.get_idx(),
                                    *tree_pos,
                                    &*rlock
                                );
                                if path.success {
                                    crate::messaging::job_changed(id.id,
                                        JobType::FellTree{
                                            tree_id: *tree_id,
                                            tree_pos: *tree_pos,
                                            step: LumberjackSteps::TravelToTree{ path: path.steps }
                                        }
                                    );
                                } else {
                                    println!("No path to tree available - abandoning lumberjacking");
                                    crate::messaging::cancel_job(id.id);
                                }
                            }
                            LumberjackSteps::TravelToTree{path} => {
                                if path.len() > 1 {
                                    crate::messaging::follow_job_path(id.id);
                                } else {
                                    crate::messaging::job_changed(id.id,
                                        JobType::FellTree{
                                            tree_id: *tree_id,
                                            tree_pos: *tree_pos,
                                            step: LumberjackSteps::ChopTree{  }
                                        }
                                    );
                                }
                            }
                            LumberjackSteps::ChopTree{} => {
                                crate::messaging::chop_tree(id.id, *tree_id);
                                crate::messaging::conclude_job(id.id);
                            }
                        }
                    } else {
                        panic!("Not doing a lumberjack job but wound up in the LJ system!");
                    }
                }
            );
        }
    )
}
