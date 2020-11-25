use super::messaging;
use super::REGION;
use legion::world::SubWorld;
use legion::*;
use nox_components::*;
use nox_planet::pathfinding::a_star_search;

#[system()]
#[read_component(MyTurn)]
#[read_component(Position)]
#[read_component(IdentityTag)]
#[read_component(Construction)]
#[read_component(Blueprint)]
pub fn construction(ecs: &SubWorld) {
    let mut bquery = <(&MyTurn, &Position, &IdentityTag)>::query();
    bquery.iter(ecs).for_each(|(turn, pos, id)| {
        if turn.active
            && turn.shift == ScheduleTime::Work
            && match turn.job {
                JobType::Construct { .. } => true,
                _ => false,
            }
        {
            println!("Construction mode");
            if let JobType::Construct { building_id, step } = &turn.job {
                match step {
                    ConstructionSteps::FindBuilding => {
                        println!("Finding building");
                        let bpos = <(&IdentityTag, &Position)>::query()
                            .filter(component::<Construction>())
                            .iter(ecs)
                            .filter(|(id, _pos)| id.0 == *building_id)
                            .map(|(_id, pos)| pos.get_idx())
                            .nth(0)
                            .unwrap();

                        let rlock = REGION.read();
                        let start = pos.get_idx();
                        println!("Pathing from {} to {}", start, bpos);
                        let path = a_star_search(start, bpos, &rlock);
                        std::mem::drop(rlock);
                        if path.success {
                            messaging::job_changed(
                                id.0,
                                JobType::Construct {
                                    building_id: *building_id,
                                    step: ConstructionSteps::TravelToBuilding { path: path.steps },
                                },
                            );
                        } else {
                            // Abandon all hope
                            println!("Unable to find building");
                        }
                    }
                    ConstructionSteps::TravelToBuilding { path } => {
                        println!("Following path");
                        if path.len() > 1 {
                            messaging::follow_job_path(id.0);
                        } else {
                            // We've arrived.
                            messaging::job_changed(
                                id.0,
                                JobType::Construct {
                                    building_id: *building_id,
                                    step: ConstructionSteps::Construct,
                                },
                            );
                        }
                    }
                    ConstructionSteps::Construct => {
                        println!("Construction done");
                        messaging::finish_construction(*building_id);
                        messaging::conclude_job(id.0);
                    }
                }
            }
        }
    })
}
