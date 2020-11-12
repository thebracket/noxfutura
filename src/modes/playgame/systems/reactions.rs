use super::super::messaging;
use crate::modes::playgame::systems::REGION;
use legion::*;
use nox_components::*;
use nox_planet::pathfinding::a_star_search;

#[system(for_each)]
pub fn reactions(turn: &MyTurn, pos: &Position, id: &IdentityTag, _settler: &Settler) {
    if turn.active
        && turn.shift == ScheduleTime::Work
        && match turn.job {
            JobType::Reaction { .. } => true,
            _ => false,
        }
    {
        if let JobType::Reaction {
            workshop_id,
            workshop_pos,
            reaction_id,
            components,
            step,
        } = &turn.job {
            match step {
                ReactionSteps::ClaimEverything => {
                    let mut rlock = REGION.write();
                    for (cid, cpos, _, _) in components.iter() {
                        if rlock.jobs_board.is_component_claimed(*cid) {
                            rlock.jobs_board.claim_component_for_building(*workshop_id, *cid, *cpos);
                        } else {
                            // TODO: Cancel Job
                        }
                    }
                    messaging::job_changed(
                        id.0,
                        JobType::Reaction {
                            workshop_id: *workshop_id,
                            workshop_pos: *workshop_pos,
                            reaction_id: *reaction_id,
                            components: components.clone(),
                            step: ReactionSteps::FindComponent
                        },
                    );
                }
                ReactionSteps::FindComponent => {
                    if let Some(next_component) = components
                        .iter()
                        .filter(|(_pos, _id, claimed, _mat)| !claimed)
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
                            // TODO: Cancel
                        } else {
                            // Path onwards
                            messaging::job_changed(
                                id.0,
                                JobType::Reaction {
                                    workshop_id: *workshop_id,
                                    workshop_pos: *workshop_pos,
                                    reaction_id: *reaction_id,
                                    components: components.clone(),
                                    step: ReactionSteps::TravelToComponent {
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
                            JobType::Reaction {
                                workshop_id: *workshop_id,
                                workshop_pos: *workshop_pos,
                                reaction_id: *reaction_id,
                                components: components.clone(),
                                step: ReactionSteps::Construct {},
                            },
                        );
                    }
                }
                ReactionSteps::TravelToComponent { path, component_id } => {
                    println!("Following path");
                    if path.len() > 1 {
                        messaging::follow_job_path(id.0);
                        println!("My pos: {}", pos.get_idx());
                    } else {
                        messaging::job_changed(
                            id.0,
                            JobType::Reaction {
                                workshop_id: *workshop_id,
                                workshop_pos: *workshop_pos,
                                reaction_id: *reaction_id,
                                components: components.clone(),
                                step: ReactionSteps::CollectComponent {
                                    component_id: *component_id,
                                },
                            },
                        );
                    }
                }
                ReactionSteps::CollectComponent { component_id } => {
                    println!("Component Collected");
                    //messaging::equip_tool(id.0, *component_id);
                    messaging::job_changed(
                        id.0,
                        JobType::Reaction {
                            workshop_id: *workshop_id,
                            workshop_pos: *workshop_pos,
                            reaction_id: *reaction_id,
                            components: components.clone(),
                            step: ReactionSteps::FindWorkshop {
                                component_id: *component_id,
                            },
                        },
                    );
                }
                ReactionSteps::FindWorkshop { component_id } => {
                    println!("Finding the workshop site");
                    let rlock = REGION.read();
                    println!("Building pos: {}", workshop_pos);
                    println!("My pos: {}", pos.get_idx());
                    let path = a_star_search(pos.get_idx(), *workshop_pos, &*rlock);
                    if path.success {
                        println!("Path to building established");
                        messaging::job_changed(
                            id.0,
                            JobType::Reaction {
                                workshop_id: *workshop_id,
                                workshop_pos: *workshop_pos,
                                reaction_id: *reaction_id,
                                components: components.clone(),
                                step: ReactionSteps::TravelToWorkshop {
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
                        // TODO: Cancel
                    }
                }
                ReactionSteps::TravelToWorkshop { component_id, path } => {
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
                        new_components.iter_mut().for_each(|(idx, id, complete, _mat)| {
                            if id == component_id {
                                *complete = true;
                                *idx = pos.get_idx();
                            }
                        });
                        println!("{:#?}", new_components);

                        // Return to find components
                        messaging::job_changed(
                            id.0,
                            JobType::Reaction {
                                workshop_id: *workshop_id,
                                workshop_pos: *workshop_pos,
                                reaction_id: *reaction_id,
                                components: components.clone(),
                                step: ReactionSteps::FindComponent,
                            },
                        );
                    }
                }
                ReactionSteps::Construct => {
                    use nox_raws::*;
                    for out in RAWS.read().reactions.reactions[*reaction_id].outputs.iter() {
                        // tag/qty
                        messaging::spawn_item(workshop_pos, &out.tag, &out.qty, components[0].3);
                    }
                    components.iter().for_each(|(_idx, id, _claimed, _mat)| {
                        REGION
                            .write()
                            .jobs_board
                            .relinquish_component_for_building(*id);
                        messaging::delete_item(*id);
                    });
                    // TODO: Spawn results
                }
            }
        }
    }
}