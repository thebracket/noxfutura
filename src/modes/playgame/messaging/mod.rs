mod queue_backend;
mod job_step;
mod apply;

use bracket_geometry::prelude::Point3;
use queue_backend::*;
use job_step::*;
use legion::*;

use crate::components::JobType;

pub fn process_queues(ecs: &mut World, resources: &mut Resources) {
    apply::apply_jobs_queue(ecs, resources);
}

pub fn entity_moved(id: usize, end: &Point3) {
    JOBS_QUEUE.lock().push_back(JobStep::EntityMoved {
        id,
        end: end.clone(),
    });
}

pub fn cancel_job(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::JobCancelled { id });
}

pub fn relinquish_claim(tool_id: usize, tool_pos: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::RelinquishClaim { tool_id, tool_pos });
}

pub fn job_changed(id: usize, new_job: JobType) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::JobChanged { id, new_job });
}

pub fn follow_job_path(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::FollowJobPath { id });
}

pub fn equip_tool(id: usize, tool_id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::EquipItem { id, tool_id });
}

pub fn drop_item(id: usize, location: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::DropItem { id, location });
}

pub fn chop_tree(id: usize, tree_id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::TreeChop { id, tree_id });
}

pub fn conclude_job(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::JobConcluded { id });
}

pub fn vox_moved() {
    JOBS_QUEUE.lock().push_back(JobStep::VoxMoved);
}

pub fn models_moved() {
    JOBS_QUEUE.lock().push_back(JobStep::ModelsMoved);
}