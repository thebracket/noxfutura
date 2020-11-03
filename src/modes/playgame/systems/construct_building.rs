use super::messaging;
use super::REGION;
use legion::*;
use nox_components::*;
use nox_planet::pathfinding::a_star_search;

#[system(for_each)]
pub fn construction(turn: &MyTurn, pos: &Position, id: &IdentityTag) {
    if turn.active
        && turn.shift == ScheduleTime::Work
        && match turn.job {
            JobType::ConstructBuilding { .. } => true,
            _ => false,
        }
    {
        if let JobType::ConstructBuilding {
            building_id,
            building_pos,
            step,
            components,
        } = &turn.job
        {
            match step {
                BuildingSteps::FindComponent => {
                    println!("{:#?}", components);
                    if let Some(next_component) = components
                        .iter()
                        .filter(|(_pos, _id, claimed)| !claimed)
                        .nth(0)
                    {
                        println!("Finding component");
                        // Path to component
                        let rlock = REGION.read();
                        let path = a_star_search(
                            pos.get_idx(),
                            next_component.0, // REPLACEME with component position
                            &*rlock,
                        );
                        if !path.success {
                            println!("No path to component available - abandoning building");
                            messaging::cancel_job(id.0);
                            for c in components.iter() {
                                REGION
                                    .write()
                                    .jobs_board
                                    .relinquish_component_for_building(c.1);
                            }
                            messaging::delete_building(*building_id);
                        } else {
                            // Path onwards
                            messaging::job_changed(
                                id.0,
                                JobType::ConstructBuilding {
                                    building_id: *building_id,
                                    building_pos: *building_pos,
                                    components: components.clone(),
                                    step: BuildingSteps::TravelToComponent {
                                        path: path.steps,
                                        component_id: next_component.1,
                                    },
                                },
                            );
                        }
                    } else {
                        // Components are present - build it
                        println!("Building building");
                        messaging::job_changed(
                            id.0,
                            JobType::ConstructBuilding {
                                building_id: *building_id,
                                building_pos: *building_pos,
                                components: components.clone(),
                                step: BuildingSteps::Construct {},
                            },
                        );
                    }
                }
                BuildingSteps::TravelToComponent { path, component_id } => {
                    println!("Following path");
                    if path.len() > 1 {
                        messaging::follow_job_path(id.0);
                        println!("My pos: {}", pos.get_idx());
                    } else {
                        messaging::job_changed(
                            id.0,
                            JobType::ConstructBuilding {
                                building_id: *building_id,
                                building_pos: *building_pos,
                                components: components.clone(),
                                step: BuildingSteps::CollectComponent {
                                    component_id: *component_id,
                                },
                            },
                        );
                    }
                }
                BuildingSteps::CollectComponent { component_id } => {
                    println!("Component Collected");
                    //messaging::equip_tool(id.0, *component_id);
                    messaging::job_changed(
                        id.0,
                        JobType::ConstructBuilding {
                            building_id: *building_id,
                            building_pos: *building_pos,
                            components: components.clone(),
                            step: BuildingSteps::FindBuilding {
                                component_id: *component_id,
                            },
                        },
                    );
                }
                BuildingSteps::FindBuilding { component_id } => {
                    println!("Finding the building site");
                    let rlock = REGION.read();
                    println!("Building pos: {}", building_pos);
                    println!("My pos: {}", pos.get_idx());
                    let path = a_star_search(pos.get_idx(), *building_pos, &*rlock);
                    if path.success {
                        println!("Path to building established");
                        messaging::job_changed(
                            id.0,
                            JobType::ConstructBuilding {
                                building_id: *building_id,
                                building_pos: *building_pos,
                                components: components.clone(),
                                step: BuildingSteps::TravelToTBuilding {
                                    path: path.steps,
                                    component_id: *component_id,
                                },
                            },
                        );
                    } else {
                        println!("No path to building");
                        // Cancel job
                        messaging::cancel_job(id.0);
                        for c in components.iter() {
                            REGION
                                .write()
                                .jobs_board
                                .relinquish_component_for_building(c.1);
                        }
                        messaging::delete_building(*building_id);
                    }
                }
                BuildingSteps::TravelToTBuilding { component_id, path } => {
                    println!("Following path");
                    if path.len() > 1 {
                        messaging::follow_job_path(id.0);
                    } else {
                        println!("We're here! Component ID: {}", component_id);
                        // We've arrived.
                        // Drop the component
                        messaging::drop_item(*component_id, pos.get_idx());
                        // Update the components vector
                        let mut new_components = components.clone();
                        new_components.iter_mut().for_each(|(idx, id, complete)| {
                            if id == component_id {
                                *complete = true;
                                *idx = pos.get_idx();
                            }
                        });
                        println!("{:#?}", new_components);

                        // Return to find components
                        messaging::job_changed(
                            id.0,
                            JobType::ConstructBuilding {
                                building_id: *building_id,
                                building_pos: *building_pos,
                                components: new_components,
                                step: BuildingSteps::FindComponent,
                            },
                        );
                    }
                }
                BuildingSteps::Construct => {
                    println!("Building done");
                    messaging::conclude_job(id.0);
                    components.iter().for_each(|(_idx, id, _claimed)| {
                        REGION
                            .write()
                            .jobs_board
                            .relinquish_component_for_building(*id);
                        messaging::delete_item(*id);
                    });
                    messaging::finish_building(*building_id);
                }
            }
        }
    }
}
