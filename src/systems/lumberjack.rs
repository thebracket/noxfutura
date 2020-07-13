use legion::prelude::*;
use nox_components::*;
use crate::systems::REGION;
use bracket_pathfinding::prelude::a_star_search;
use nox_planet::{mapidx, idxmap};

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("lumberjack")
        .with_query(<(Write<MyTurn>, Write<Position>, Read<Identity>)>::query())
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
                .for_each(|(mut turn, mut pos, id)| {
                    turn.order = WorkOrder::None;

                    if let JobType::FellTree{ tree_id, tree_pos, step } = &mut turn.job {
                        match step {
                            LumberjackSteps::FindAxe => {
                                let mut rlock = REGION.write();
                                let maybe_tool_pos = rlock.jobs_board.find_and_claim_tool(ToolType::Chopping, id.id);
                                if let Some(tool_pos) = maybe_tool_pos {
                                    let path = a_star_search(
                                        mapidx(pos.x, pos.y, pos.z), 
                                        tool_pos, 
                                        &*rlock
                                    );
                                    if !path.success {
                                        println!("No path to tool available - abandoning lumberjacking");
                                        rlock.jobs_board.restore_job(&turn.job);
                                        turn.job = JobType::None;
                                    } else {
                                        *step = LumberjackSteps::TravelToAxe{ path: path.steps  };
                                    }
                                } else {
                                    println!("No tool available - abandoning lumberjacking");
                                    rlock.jobs_board.restore_job(&turn.job);
                                    turn.job = JobType::None;
                                }

                            }
                            LumberjackSteps::TravelToAxe{path} => {
                                if path.len() > 1 {
                                    let next_pos = path[0];
                                    let (nx, ny, nz) = idxmap(next_pos);
                                    path.remove(0);
                                    pos.x = nx;
                                    pos.y = ny;
                                    pos.z = nz;
                                    crate::messaging::vox_moved();
                                } else {
                                    println!("We're adjacent to the target item now. Pretending we picked it up");
                                    *step = LumberjackSteps::FindTree{  };
                                }
                            }
                            LumberjackSteps::FindTree{} => {
                                println!("Tree pos: {}", tree_pos);
                                let rlock = REGION.read();
                                let path = a_star_search(
                                    mapidx(pos.x, pos.y, pos.z),
                                    *tree_pos,
                                    &*rlock
                                );
                                if path.success {
                                    *step = LumberjackSteps::TravelToTree{ path: path.steps }
                                } else {
                                    println!("No path to tree available - abandoning lumberjacking");
                                    let mut rlock = REGION.write();
                                    // TODO: Drop the tool
                                    rlock.jobs_board.restore_job(&turn.job);
                                    turn.job = JobType::None;
                                }
                            }
                            LumberjackSteps::TravelToTree{path} => {
                                if path.len() > 1 {
                                    let next_pos = path[0];
                                    let (nx, ny, nz) = idxmap(next_pos);
                                    path.remove(0);
                                    pos.x = nx;
                                    pos.y = ny;
                                    pos.z = nz;
                                    crate::messaging::vox_moved();
                                } else {
                                    println!("We're adjacent to the target item now. Pretending we did something");
                                    *step = LumberjackSteps::ChopTree{  };
                                }
                            }
                            LumberjackSteps::ChopTree{} => {
                                println!("Done with chopping.");
                                    let mut rlock = REGION.write();
                                    // TODO: Drop the tool
                                    rlock.jobs_board.restore_job(&turn.job);
                                    turn.job = JobType::None;
                            }
                            _ => println!("Warning - LumberJack fell through with no steps.")
                        }
                    } else {
                        panic!("Not doing a lumberjack job but wound up in the LJ system!");
                    }
                }
            );
        }
    )
}
