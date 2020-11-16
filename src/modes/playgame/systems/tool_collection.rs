use super::messaging;
use legion::world::SubWorld;
use legion::*;
use nox_components::*;

#[system]
#[read_component(MyTurn)]
#[read_component(Position)]
#[read_component(IdentityTag)]
#[read_component(Settler)]
pub fn tool_collection(ecs: &SubWorld) {
    let mut lquery = <(&MyTurn, &IdentityTag)>::query();
    lquery.iter(ecs).for_each(|(turn, id)| {
        if turn.active
            && turn.shift == ScheduleTime::Work
            && match turn.job {
                JobType::CollectTool { .. } => true,
                _ => false,
            }
        {
            if let JobType::CollectTool { tool_id, step } = &turn.job {
                match step {
                    CollectToolSteps::TravelToTool { path } => {
                        println!("Travel to tool");
                        if path.len() > 1 {
                            messaging::follow_job_path(id.0);
                        } else {
                            messaging::job_changed(
                                id.0,
                                JobType::CollectTool {
                                    step: CollectToolSteps::CollectTool,
                                    tool_id: *tool_id,
                                },
                            );
                        }
                    }
                    CollectToolSteps::CollectTool => {
                        println!("Collect tool");
                        messaging::equip_tool(id.0, *tool_id);
                        messaging::conclude_job(id.0);
                    }
                }
            }
        }
    });
}
