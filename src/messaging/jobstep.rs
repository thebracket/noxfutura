use crate::systems::REGION;
use bracket_geometry::prelude::Point3;
use legion::prelude::*;
use nox_components::*;
use nox_spatial::idxmap;
use std::collections::HashMap;
pub enum JobStep {
    EntityMoved { id: usize, end: Point3 },
    JobChanged { id: usize, new_job: JobType },
    JobCancelled { id: usize },
    JobConcluded { id: usize },
    FollowJobPath { id: usize },
    DropItem { id: usize, location: usize },
    RelinquishClaim { tool_id: usize },
}

pub fn apply_jobs_queue(ecs: &mut World) {
    let mut jlock = super::JOBS_QUEUE.lock();

    let mut movers: HashMap<usize, (usize, usize, usize)> = HashMap::new();

    jlock.iter().for_each(|js| {
        match js {
            JobStep::EntityMoved { id, end } => {
                movers.insert(*id, (end.x as usize, end.y as usize, end.z as usize));
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
                                movers.insert(*id, (x, y, z));
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
        }
    });

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

    // Clear the queue
    jlock.clear();
}
