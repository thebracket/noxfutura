use crate::systems::REGION;
use bracket_geometry::prelude::Point3;
use legion::prelude::*;
use nox_components::*;
use nox_spatial::idxmap;
use super::MOVER_LIST;

pub enum JobStep {
    EntityMoved { id: usize, end: Point3 },
    JobChanged { id: usize, new_job: JobType },
    JobCancelled { id: usize },
    JobConcluded { id: usize },
    FollowJobPath { id: usize },
    DropItem { id: usize, location: usize },
    RelinquishClaim { tool_id: usize },
    EquipItem { id: usize, tool_id: usize },
    TreeChop { id: usize, tree_id: usize },
}

pub fn apply_jobs_queue(ecs: &mut World) {
    MOVER_LIST.lock().clear();
    loop {
        let js = super::JOBS_QUEUE.lock().pop_front();
        if let Some(mut js) = js {
            apply(ecs, &mut js);
        } else {
            break;
        }
    }
}

fn apply(ecs: &mut World, js: &mut JobStep) {
    match js {
        JobStep::EntityMoved { id, end } => {
            MOVER_LIST.lock().insert(*id, (end.x as usize, end.y as usize, end.z as usize));
        }
        JobStep::JobChanged { id, new_job } => {
            let idtag = IdentityTag(*id);
            <Write<MyTurn>>::query()
                .filter(tag_value(&idtag))
                .iter_mut(ecs)
                .for_each(|mut turn| {
                    turn.job = new_job.clone();
                });
        }
        JobStep::JobCancelled { id } => {
            let idtag = IdentityTag(*id);
            <Write<MyTurn>>::query()
                .filter(tag_value(&idtag))
                .iter_mut(ecs)
                .for_each(|mut turn| {
                    REGION.write().jobs_board.restore_job(&turn.job);
                    turn.job = JobType::None;
                });
        }
        JobStep::JobConcluded { id } => {
            println!("Job finished");
            let idtag = IdentityTag(*id);
            <Write<MyTurn>>::query()
                .filter(tag_value(&idtag))
                .iter_mut(ecs)
                .for_each(|mut turn| {
                    match &turn.job {
                        JobType::FellTree {
                            tree_id,
                            tree_pos: _,
                            step: _,
                            tool_id: _,
                        } => {
                            // Un-designate the tree
                            let mut rlock = REGION.write();
                            rlock.jobs_board.remove_tree(&tree_id);
                        }
                        _ => {}
                    }
                    turn.job = JobType::None;
                });
        }
        JobStep::FollowJobPath { id } => {
            let idtag = IdentityTag(*id);
            <Write<MyTurn>>::query()
                .filter(tag_value(&idtag))
                .iter_mut(ecs)
                .for_each(|mut turn| match &mut turn.job {
                    JobType::FellTree { step, .. } => {
                        let path = match step {
                            LumberjackSteps::TravelToAxe { path, .. } => Some(path),
                            LumberjackSteps::TravelToTree { path } => Some(path),
                            _ => None,
                        };
                        if let Some(path) = path {
                            let destination = path[0];
                            path.remove(0);
                            let (x, y, z) = idxmap(destination);
                            MOVER_LIST.lock().insert(*id, (x, y, z));
                        }
                    }
                    _ => {}
                });
        }
        JobStep::DropItem { id, location } => {
            let idtag = IdentityTag(*id);
            <Write<Position>>::query()
                .filter(tag_value(&idtag))
                .iter_mut(ecs)
                .for_each(|mut pos| {
                    pos.to_ground(*location);
                });
        }
        JobStep::RelinquishClaim { tool_id } => {
            REGION.write().jobs_board.relinquish_claim(*tool_id);
        }
        JobStep::TreeChop { id: _, tree_id } => {
            println!("Chop tree");
            // TODO: Remove the tree from the region data
            let treetag = IdentityTag(*tree_id);
            let tree_pos = <Read<Position>>::query()
                .filter(tag_value(&treetag))
                .iter_entities(ecs)
                .map(|(e, pos)| (e, pos.get_idx()))
                .nth(0)
                .unwrap();
            let (tx, ty, tz) = idxmap(tree_pos.1);

            ecs.delete(tree_pos.0);

            let mut rlock = crate::systems::REGION.write();
            for _ in 0..crate::systems::RNG.lock().roll_dice(1, 6) {
                nox_planet::spawn_item_on_ground(ecs, "wood_log", tx, ty, tz, &mut *rlock);
            }
            super::vox_moved();
        }

        JobStep::EquipItem { id, tool_id } => {
            let itemtag = IdentityTag(*tool_id);
            <Write<Position>>::query()
                .filter(tag_value(&itemtag))
                .iter_mut(ecs)
                .for_each(|mut pos| {
                    pos.to_carried(*id);
                });
            super::vox_moved();
        }
    };

    let movers = MOVER_LIST.lock();
    if !movers.is_empty() {
        super::vox_moved();
        <(Tagged<IdentityTag>, Write<Position>)>::query()
            .iter_mut(ecs)
            .filter(|(id, _)| movers.contains_key(&id.0))
            .for_each(|(id, mut pos)| {
                if let Some(destination) = movers.get(&id.0) {
                    pos.set_tile_loc(destination);
                }
            });
    }
}
