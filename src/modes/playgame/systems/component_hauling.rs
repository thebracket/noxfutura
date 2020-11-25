use super::messaging;
use super::REGION;
use legion::world::SubWorld;
use legion::*;
use nox_components::*;
use nox_planet::pathfinding::a_star_search;

#[system]
#[read_component(MyTurn)]
#[read_component(Position)]
#[read_component(IdentityTag)]
#[read_component(Settler)]
#[read_component(RequestHaul)]
pub fn hauling(ecs: &SubWorld) {
    let mut lquery = <(&MyTurn, &IdentityTag, &Position)>::query();
    lquery.iter(ecs).for_each(|(turn, id, pos)| {
        if turn.active
            && turn.shift == ScheduleTime::Work
            && match turn.job {
                JobType::Haul { .. } => true,
                _ => false,
            }
        {
            if let JobType::Haul { item_id, step } = &turn.job {
                match step {
                    HaulSteps::FindItem => {
                        <(&RequestHaul, &Position, &IdentityTag)>::query()
                            .iter(ecs)
                            .filter(|(_, _, hid)| hid.0 == *item_id)
                            .for_each(|(_rh, hpos, _hid)| {
                                let destination = hpos.effective_location_sw(ecs);
                                let path =
                                    a_star_search(pos.get_idx(), destination, &REGION.read());
                                if path.success {
                                    messaging::job_changed(
                                        id.0,
                                        JobType::Haul {
                                            item_id: *item_id,
                                            step: HaulSteps::TravelToItem { path: path.steps },
                                        },
                                    );
                                } else {
                                    // Cancel job
                                }
                            });
                    }
                    HaulSteps::TravelToItem { path } => {
                        if path.len() > 1 {
                            messaging::follow_job_path(id.0);
                        } else {
                            messaging::job_changed(
                                id.0,
                                JobType::Haul {
                                    item_id: *item_id,
                                    step: HaulSteps::CollectItem,
                                },
                            );
                        }
                    }
                    HaulSteps::CollectItem => {
                        messaging::get_item(id.0, *item_id);
                        <(&RequestHaul, &Position, &IdentityTag)>::query()
                            .iter(ecs)
                            .filter(|(_, _, hid)| hid.0 == *item_id)
                            .for_each(|(rh, _hpos, _)| {
                                let destination = rh.destination;
                                let path =
                                    a_star_search(pos.get_idx(), destination, &REGION.read());
                                if path.success {
                                    messaging::job_changed(
                                        id.0,
                                        JobType::Haul {
                                            item_id: *item_id,
                                            step: HaulSteps::TravelToDestination {
                                                path: path.steps,
                                            },
                                        },
                                    );
                                } else {
                                    // Cancel job
                                }
                            });
                    }
                    HaulSteps::TravelToDestination { path } => {
                        if path.len() > 1 {
                            messaging::follow_job_path(id.0);
                        } else {
                            messaging::job_changed(
                                id.0,
                                JobType::Haul {
                                    item_id: *item_id,
                                    step: HaulSteps::DropItem,
                                },
                            );
                        }
                    }
                    HaulSteps::DropItem => {
                        let destination = <(&RequestHaul, &Position, &IdentityTag)>::query()
                            .iter(ecs)
                            .filter(|(_, _, hid)| hid.0 == *item_id)
                            .map(|(_, hpos, _)| hpos.effective_location_sw(ecs))
                            .nth(0)
                            .unwrap();
                        messaging::drop_item(*item_id, destination);
                        messaging::update_blueprint(*item_id);
                        messaging::conclude_job(id.0);
                    }
                }
            }
        }
    });
}
