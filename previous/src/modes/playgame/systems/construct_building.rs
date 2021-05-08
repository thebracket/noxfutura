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
#[read_component(Building)]
#[read_component(Blueprint)]
pub fn construction_building(ecs: &SubWorld) {
    let mut bquery = <(&MyTurn, &Position, &IdentityTag)>::query();
    bquery.iter(ecs).for_each(|(turn, pos, id)| {
        if turn.active
            && turn.shift == ScheduleTime::Work
            && match turn.job {
                JobType::ConstructBuilding { .. } => true,
                _ => false,
            }
        {
            if let JobType::ConstructBuilding { building_id, step } = &turn.job {
                match step {
                    BuildingSteps::FindBuilding => {
                        let bpos = <(&IdentityTag, &Position)>::query()
                            .filter(component::<Building>())
                            .iter(ecs)
                            .filter(|(id, _pos)| id.0 == *building_id)
                            .map(|(_id, pos)| pos.get_idx())
                            .nth(0)
                            .unwrap();

                        let rlock = REGION.read();
                        let start = pos.get_idx();
                        let path = a_star_search(start, bpos, &rlock);
                        std::mem::drop(rlock);
                        if path.success {
                            messaging::job_changed(
                                id.0,
                                JobType::ConstructBuilding {
                                    building_id: *building_id,
                                    step: BuildingSteps::TravelToBuilding { path: path.steps },
                                },
                            );
                        } else {
                            // Abandon all hope
                            println!("Unable to find building");
                        }
                    }
                    BuildingSteps::TravelToBuilding { path } => {
                        if path.len() > 1 {
                            messaging::follow_job_path(id.0);
                        } else {
                            // We've arrived.
                            messaging::job_changed(
                                id.0,
                                JobType::ConstructBuilding {
                                    building_id: *building_id,
                                    step: BuildingSteps::Construct,
                                },
                            );
                        }
                    }
                    BuildingSteps::Construct => {
                        messaging::finish_building(*building_id);
                        messaging::conclude_job(id.0);
                    }
                }
            }
        }
    })
}
