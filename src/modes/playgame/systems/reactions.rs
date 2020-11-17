use super::super::messaging;
use crate::modes::playgame::systems::REGION;
use legion::world::SubWorld;
use legion::*;
use nox_components::*;
use nox_planet::pathfinding::a_star_search;
use nox_spatial::idxmap;

#[system]
#[read_component(MyTurn)]
#[read_component(Position)]
#[read_component(IdentityTag)]
#[read_component(Settler)]
#[read_component(ReactionJob)]
pub fn reactions(ecs: &SubWorld) {
    <(&MyTurn, &Position, &IdentityTag, &Settler)>::query()
        .iter(ecs)
        .for_each(|(turn, pos, id, settler)| {
            if turn.active
                && turn.shift == ScheduleTime::Work
                && match turn.job {
                    JobType::Reaction { .. } => true,
                    _ => false,
                }
            {
                if let JobType::Reaction {
                    reaction_id,
                    reaction_location,
                    step,
                } = &turn.job
                {
                    println!("Found a reaction job");
                    match step {
                        ReactionSteps::FindReaction => {
                            let start = pos.get_idx();
                            println!("Search for position of reaction {}", reaction_id);
                            let end = *reaction_location;
                            println!("Reaction: {:?}", idxmap(end));
                            println!("Settler: {:?}", idxmap(pos.get_idx()));

                            let rlock = REGION.read();
                            let path = a_star_search(start, end, &rlock);
                            if path.success {
                                messaging::job_changed(
                                    id.0,
                                    JobType::Reaction {
                                        reaction_id: *reaction_id,
                                        reaction_location: *reaction_location,
                                        step: ReactionSteps::TravelToReaction { path: path.steps },
                                    },
                                );
                            } else {
                                // Abandon job
                                println!("Failed to find path to perform reaction");
                            }
                        }
                        ReactionSteps::TravelToReaction { path } => {
                            println!("Travel to destination");
                            if path.len() > 1 {
                                messaging::follow_job_path(id.0);
                            } else {
                                messaging::job_changed(
                                    id.0,
                                    JobType::Reaction {
                                        reaction_id: *reaction_id,
                                        reaction_location: *reaction_location,
                                        step: ReactionSteps::PerformReaction,
                                    },
                                );
                            }
                        }
                        ReactionSteps::PerformReaction => {
                            println!("Perform reaction");
                            messaging::perform_reaction(*reaction_id);
                            messaging::conclude_job(id.0);
                        }
                    }
                }
            }
        });
}
