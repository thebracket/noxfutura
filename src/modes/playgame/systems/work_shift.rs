use super::REGION;
use legion::*;
use legion::world::SubWorld;
use nox_components::*;
use nox_planet::{MiningMap, LumberMap};
use nox_spatial::*;
use bengine::geometry::*;

#[system]
#[write_component(MyTurn)]
#[read_component(Settler)]
#[read_component(Position)]
#[read_component(IdentityTag)]
#[read_component(RequestHaul)]
pub fn work_shift(
    ecs: &mut SubWorld,
    #[resource] mining: &MiningMap,
    #[resource] lumber: &LumberMap
) {
    let haulables = haulage_list(ecs);
    <(&mut MyTurn, &Settler, &Position, &IdentityTag)>::query().iter_mut(ecs).for_each(| (turn, settler, pos, id) | {
        if turn.active && turn.shift == ScheduleTime::Work && turn.job == JobType::None {
            turn.order = WorkOrder::None;
            let settler_pos = pos.get_idx();

            let mut possible_jobs : Vec<(f32, JobType)> = Vec::new();

            if let Some(mining_cost) = consider_mining(settler, mining, settler_pos) {
                possible_jobs.push((mining_cost, JobType::Mining {
                    step: MiningSteps::FindPick,
                    tool_id: None,
                }));
            }
            if let Some(lumber_cost) = consider_lumber(settler, lumber, settler_pos) {
                possible_jobs.push((lumber_cost, JobType::FellTree {
                    step: LumberjackSteps::FindAxe,
                    tool_id: None,
                }));
            }
            if let Some(haul_cost) = consider_hauling(&haulables, pos.as_point3()) {
                possible_jobs.push((haul_cost.0, JobType::Haul {
                    item_id: haul_cost.1
                }));
            }
            // TODO: Build
            // TODO: Reactions

            if possible_jobs.is_empty() {
                turn.order = WorkOrder::MoveRandomly;
            } else {
                possible_jobs.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());
                turn.job = possible_jobs[0].1.clone();
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
        .map(|(_, pos, id)|
            (
                pos.get_idx(),
                id.0
            )
        )
        .collect()
}

fn consider_hauling(haulables: &[(usize, usize)], settler_pos: Point3) -> Option<(f32, usize)> {
    let mut hsort : Vec<(f32, usize)> = haulables
        .iter()
        .map(|(pos, id)| {
            let (x, y, z) = idxmap(*pos);
            (
                DistanceAlg::Pythagoras.distance3d(Point3::new(x,y,z), settler_pos),
                *id
            )
        })
        .collect();
    if hsort.is_empty() {
        None
    } else {
        hsort.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());
        Some(hsort[0])
    }
}