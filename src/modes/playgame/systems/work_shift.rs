use crate::modes::playgame::messaging;

use bengine::geometry::*;
use legion::world::SubWorld;
use legion::*;
use nox_components::*;
use nox_planet::{LumberMap, MiningMap};
use nox_spatial::*;

#[system]
#[write_component(MyTurn)]
#[read_component(Settler)]
#[read_component(Position)]
#[read_component(IdentityTag)]
#[read_component(RequestHaul)]
#[read_component(Blueprint)]
#[read_component(Building)]
#[read_component(ReactionJob)]
pub fn work_shift(
    ecs: &mut SubWorld,
    #[resource] mining: &MiningMap,
    #[resource] lumber: &LumberMap,
) {
    let mut haulables = haulage_list(ecs);
    let buildables = building_list(ecs);
    let mut reactions = reactions_list(ecs);
    <(&mut MyTurn, &Settler, &Position, &IdentityTag)>::query()
        .iter_mut(ecs)
        .for_each(|(turn, settler, pos, id)| {
            if turn.active && turn.shift == ScheduleTime::Work && turn.job == JobType::None {
                turn.order = WorkOrder::None;
                let settler_pos = pos.get_idx();

                let mut possible_jobs: Vec<(f32, JobType)> = Vec::new();

                if let Some(mining_cost) = consider_mining(settler, mining, settler_pos) {
                    possible_jobs.push((
                        mining_cost,
                        JobType::Mining {
                            step: MiningSteps::FindPick,
                            tool_id: None,
                        },
                    ));
                }
                if let Some(lumber_cost) = consider_lumber(settler, lumber, settler_pos) {
                    possible_jobs.push((
                        lumber_cost,
                        JobType::FellTree {
                            step: LumberjackSteps::FindAxe,
                            tool_id: None,
                        },
                    ));
                }
                if let Some(haul_cost) = consider_hauling(&haulables, pos.as_point3()) {
                    possible_jobs.push((
                        haul_cost.0,
                        JobType::Haul {
                            item_id: haul_cost.1,
                            step: HaulSteps::FindItem,
                        },
                    ));
                }
                if let Some(build_cost) = consider_building(&buildables, pos.as_point3()) {
                    possible_jobs.push((
                        build_cost.0,
                        JobType::ConstructBuilding {
                            building_id: build_cost.1,
                            step: BuildingSteps::FindBuilding,
                        },
                    ));
                }
                if let Some(reaction_cost) = consider_reactions(&reactions, pos.as_point3()) {
                    println!(
                        "Picked reaction {} at {:?}",
                        reaction_cost.1, reaction_cost.2
                    );
                    possible_jobs.push((
                        reaction_cost.0,
                        JobType::Reaction {
                            reaction_id: reaction_cost.1,
                            reaction_location: reaction_cost.2,
                            step: ReactionSteps::FindReaction,
                        },
                    ));
                }

                if possible_jobs.is_empty() {
                    turn.order = WorkOrder::MoveRandomly;
                } else {
                    possible_jobs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    turn.job = possible_jobs[0].1.clone();

                    match turn.job {
                        JobType::Haul { item_id, .. } => {
                            // Remove from haulables list
                            haulables.retain(|(_, iid)| *iid != item_id);
                            messaging::haul_in_progress(item_id, id.0);
                        }
                        JobType::Reaction { reaction_id, .. } => {
                            reactions.retain(|(_, rid)| *rid != reaction_id);
                            messaging::reaction_in_progress(reaction_id, id.0);
                        }
                        _ => {}
                    }
                }
            }
        });
}

fn consider_mining(settler: &Settler, mining: &MiningMap, pos: usize) -> Option<f32> {
    if settler.miner && mining.dijkstra[pos] < f32::MAX {
        Some(mining.dijkstra[pos])
    } else {
        None
    }
}

fn consider_lumber(settler: &Settler, lumber: &LumberMap, pos: usize) -> Option<f32> {
    if settler.lumberjack && lumber.dijkstra[pos] < f32::MAX {
        Some(lumber.dijkstra[pos])
    } else {
        None
    }
}

fn haulage_list(ecs: &SubWorld) -> Vec<(usize, usize)> {
    <(&RequestHaul, &Position, &IdentityTag)>::query()
        .iter(ecs)
        .filter(|(rh, _, _)| rh.in_progress.is_none())
        .map(|(_, pos, id)| (pos.get_idx(), id.0))
        .collect()
}

fn consider_hauling(haulables: &[(usize, usize)], settler_pos: Point3) -> Option<(f32, usize)> {
    let mut hsort: Vec<(f32, usize)> = haulables
        .iter()
        .map(|(pos, id)| {
            let (x, y, z) = idxmap(*pos);
            (
                DistanceAlg::Pythagoras.distance3d(Point3::new(x, y, z), settler_pos),
                *id,
            )
        })
        .collect();
    if hsort.is_empty() {
        None
    } else {
        hsort.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Some(hsort[0])
    }
}

fn building_list(ecs: &SubWorld) -> Vec<(usize, usize)> {
    <(&Building, &Blueprint, &Position, &IdentityTag)>::query()
        .iter(ecs)
        .filter(|(building, blueprint, _, _)| !building.complete && blueprint.ready_to_build)
        .map(|(_, _, bpos, bid)| (bpos.get_idx(), bid.0))
        .collect()
}

fn consider_building(buildables: &[(usize, usize)], settler_pos: Point3) -> Option<(f32, usize)> {
    if buildables.is_empty() {
        return None;
    }
    let mut hsort: Vec<(f32, usize)> = buildables
        .iter()
        .map(|(pos, id)| {
            let (x, y, z) = idxmap(*pos);
            (
                DistanceAlg::Pythagoras.distance3d(Point3::new(x, y, z), settler_pos),
                *id,
            )
        })
        .collect();
    if hsort.is_empty() {
        None
    } else {
        hsort.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Some(hsort[0])
    }
}

fn reactions_list(ecs: &SubWorld) -> Vec<(usize, usize)> {
    <(&ReactionJob, &IdentityTag, &Blueprint)>::query()
        .filter(!component::<Claimed>())
        .iter(ecs)
        .filter(|(rj, _id, bp)| bp.ready_to_build && rj.in_progress.is_none())
        .map(|(_rj, id, _bp)| {
            let bpos = <(&IdentityTag, &Position)>::query()
                .iter(ecs)
                .filter(|(wid, _)| wid.0 == id.0)
                .map(|(_, pos)| pos.effective_location_sw(ecs))
                .nth(0)
                .unwrap();

            (bpos, id.0)
        })
        .collect()
}

fn consider_reactions(
    reactions: &[(usize, usize)],
    settler_pos: Point3,
) -> Option<(f32, usize, usize)> {
    if reactions.is_empty() {
        return None;
    }
    let mut hsort: Vec<(f32, usize, usize)> = reactions
        .iter()
        .map(|(pos, id)| {
            let (x, y, z) = idxmap(*pos);
            (
                DistanceAlg::Pythagoras.distance3d(Point3::new(x, y, z), settler_pos),
                *id,
                *pos,
            )
        })
        .collect();
    if hsort.is_empty() {
        None
    } else {
        hsort.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Some(hsort[0])
    }
}
