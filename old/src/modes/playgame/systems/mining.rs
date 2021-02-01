use super::{
    super::messaging,
    utils::{am_i_carrying_tool, ToolCarrying},
};
use crate::modes::playgame::systems::REGION;
use bengine::geometry::Point3;
use legion::world::SubWorld;
use legion::*;
use nox_components::*;
use nox_planet::pathfinding::a_star_search;
use nox_planet::MiningMap;
use nox_spatial::idxmap;

#[system()]
#[read_component(MyTurn)]
#[read_component(Position)]
#[read_component(IdentityTag)]
#[read_component(Settler)]
#[read_component(Claimed)]
#[read_component(Tool)]
pub fn mining(ecs: &SubWorld, #[resource] mining: &MiningMap) {
    let mut mquery = <(&MyTurn, &Position, &IdentityTag, &Settler)>::query();
    mquery.iter(ecs).for_each(|(turn, pos, id, settler)| {
        if turn.active
            && turn.shift == ScheduleTime::Work
            && match turn.job {
                JobType::Mining { .. } => true,
                _ => false,
            }
        {
            if let JobType::Mining { step, tool_id } = &turn.job {
                println!("Loc at step: {:?}", pos);
                match step {
                    MiningSteps::FindPick => {
                        find_pick(ecs, id.0, pos.get_idx());
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
                                    messaging::conclude_job(id.0);
                                }
                            }
                        } else {
                            // Abort!
                            println!("Nothing to mine");
                            messaging::conclude_job(id.0);
                        }
                    }
                    MiningSteps::Dig => {
                        println!("Step: Dig");
                        messaging::dig_at(id.0, pos.get_idx());
                        messaging::conclude_job(id.0);
                        if !settler.miner {
                            messaging::drop_item(tool_id.unwrap(), pos.get_idx());
                            messaging::relinquish_claim(tool_id.unwrap(), pos.get_idx());
                        }
                    }
                }
            }
        }
    });
}

fn find_pick(ecs: &SubWorld, settler_id: usize, settler_pos: usize) {
    // Do I have an axe?
    let axe_status = am_i_carrying_tool(ecs, settler_id, ToolType::Digging);
    match axe_status {
        ToolCarrying::NoTool => messaging::cancel_job(settler_id),
        ToolCarrying::AtLocation { idx, tool_id } => {
            println!("Tool located - travel mode");
            let rlock = REGION.read();
            let path = a_star_search(settler_pos, idx, &rlock);
            if path.success {
                messaging::job_changed(
                    settler_id,
                    JobType::CollectTool {
                        step: CollectToolSteps::TravelToTool { path: path.steps },
                        tool_id,
                    },
                );
            } else {
                println!("I can't get to the pick");
                messaging::cancel_job(settler_id);
            }
        }
        ToolCarrying::Carried { tool_id } => {
            println!("I have a pick!");
            messaging::job_changed(
                settler_id,
                JobType::Mining {
                    step: MiningSteps::TravelToMine {},
                    tool_id: Some(tool_id),
                },
            );
        }
    }
}
