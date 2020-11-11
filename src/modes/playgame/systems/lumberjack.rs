use super::super::messaging;
use crate::modes::playgame::systems::REGION;
use bengine::geometry::Point3;
use legion::*;
use legion::world::SubWorld;
use nox_components::*;
use nox_planet::pathfinding::a_star_search;
use nox_spatial::idxmap;
use super::utils::{ am_i_carrying_tool, ToolCarrying };
use nox_planet::LumberMap;

#[system]
#[read_component(MyTurn)]
#[read_component(IdentityTag)]
#[read_component(Settler)]
#[read_component(Claimed)]
#[read_component(Tool)]
#[read_component(Position)]
pub fn lumberjack(ecs: &SubWorld, #[resource] lumber: &LumberMap) {
    let mut lquery = <(&MyTurn, &Position, &IdentityTag, &Settler)>::query();
    lquery.iter(ecs).for_each(|(turn, pos, id, settler)| {
        if turn.active
            && turn.shift == ScheduleTime::Work
            && match turn.job {
                JobType::FellTree { .. } => true,
                _ => false,
            }
        {
            if let JobType::FellTree {
                step,
                tool_id,
            } = &turn.job
            {
                println!("Loc at step: {:?}", pos);
                match step {
                    LumberjackSteps::FindAxe => {
                        find_axe(ecs, id.0, pos.get_idx());
                    }
                    LumberjackSteps::FindTree {} => {
                        println!("Step: FindTree");
                        travel_to_tree(id.0, pos, lumber, tool_id.unwrap());
                    }
                    LumberjackSteps::ChopTree {} => {
                        println!("Step: ChopTree");
                        messaging::chop_tree(id.0, pos.get_idx());
                        messaging::conclude_job(id.0);
                        if !settler.lumberjack {
                            messaging::drop_item(tool_id.unwrap(), pos.get_idx());
                            messaging::relinquish_claim(tool_id.unwrap(), pos.get_idx());
                        }
                    }
                }
            } else {
                panic!("Not doing a lumberjack job but wound up in the LJ system!");
            }
        }
    });

}

fn find_axe(ecs: &SubWorld, settler_id: usize, settler_pos: usize) {
    // Do I have an axe?
    let axe_status = am_i_carrying_tool(ecs, settler_id, ToolType::Chopping);
    match axe_status {
        ToolCarrying::NoTool => messaging::cancel_job(settler_id),
        ToolCarrying::AtLocation{idx, tool_id} => {
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
                println!("I can't get to the axe");
                messaging::cancel_job(settler_id);
            }
        }
        ToolCarrying::Carried{tool_id} => {
            println!("I have an axe!");
            messaging::job_changed(
                settler_id,
                JobType::FellTree {
                    step: LumberjackSteps::FindTree {},
                    tool_id: Some(tool_id),
                },
            );
        }
    }
}

fn travel_to_tree(settler_id: usize, pos: &Position, lumber: &LumberMap, tool_id: usize) {
    println!("Step: Travel to tree");
    let idx = pos.get_idx();
    let distance = lumber.dijkstra[idx];
    if distance < f32::MAX {
        println!("Distance to tree target: {}", distance);
        if distance < 1.0 {
            messaging::job_changed(
                settler_id,
                JobType::FellTree {
                    step: LumberjackSteps::ChopTree{},
                    tool_id: Some(tool_id),
                },
            );
        } else {
            // Walk the Dijkstra
            let rlock = REGION.read();
            if let Some(exit) = lumber.find_lowest_exit(idx, &rlock) {
                let (nx, ny, nz) = idxmap(exit);
                messaging::entity_moved(settler_id, &Point3::new(nx, ny, nz));
            } else {
                // Abort!
                messaging::conclude_job(settler_id);
            }
        }
    } else {
        // Abort!
        println!("Nothing to chop");
        messaging::conclude_job(settler_id);
    }
}