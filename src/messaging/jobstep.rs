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
                <(Read<Identity>, Write<MyTurn>)>::query()
                    .iter_mut(ecs)
                    .filter(|(jid, _)| jid.id == *id)
                    .for_each(|(_, mut turn)| {
                        turn.job = new_job.clone();
                    });
            }
            JobStep::JobCancelled { id } => {
                <(Read<Identity>, Write<MyTurn>)>::query()
                    .iter_mut(ecs)
                    .filter(|(jid, _)| jid.id == *id)
                    .for_each(|(_, mut turn)| {
                        crate::systems::REGION
                            .write()
                            .jobs_board
                            .restore_job(&turn.job);
                        turn.job = JobType::None;
                    });
            }
            JobStep::JobConcluded { id } => {
                <(Read<Identity>, Write<MyTurn>)>::query()
                    .iter_mut(ecs)
                    .filter(|(jid, _)| jid.id == *id)
                    .for_each(|(_, mut turn)| {
                        //TODO: Delete job
                        turn.job = JobType::None;
                    });
            }
            JobStep::FollowJobPath { id } => {
                <(Read<Identity>, Write<MyTurn>)>::query()
                    .iter_mut(ecs)
                    .filter(|(jid, _)| jid.id == *id)
                    .for_each(|(_, mut turn)| match &mut turn.job {
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
        }
    });

    if !movers.is_empty() {
        super::vox_moved();
        <(Read<Identity>, Write<Position>)>::query()
            .iter_mut(ecs)
            .filter(|(id, _)| movers.contains_key(&id.id))
            .for_each(|(id, mut pos)| {
                if let Some(destination) = movers.get(&id.id) {
                    pos.set_tile_loc(destination);
                }
            });
    }

    // Clear the queue
    jlock.clear();
}
