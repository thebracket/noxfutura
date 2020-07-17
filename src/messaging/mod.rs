mod backends;
mod renderflags;
use backends::*;
use nox_components::JobType;
mod jobstep;
use jobstep::apply_jobs_queue;
use jobstep::JobStep;
use bracket_geometry::prelude::Point3;
use legion::prelude::World;

pub use renderflags::get_render_flags;

pub fn reset() {
    RENDER_FLAGS.lock().reset();
}

pub fn process_queues(ecs: &mut World) {
    apply_jobs_queue(ecs);
}

pub fn vox_moved() {
    RENDER_FLAGS.lock().models_changed = true;
}

pub fn geometry_changed(idx: usize) {
    let mut lock = RENDER_FLAGS.lock();
    lock.terrain_changed = true;
    lock.dirty_tiles.push(idx);
}

pub fn entity_moved(id: usize, end: &Point3) {
    JOBS_QUEUE.lock().push_back(JobStep::EntityMoved {
        id,
        end: end.clone(),
    });
}

pub fn job_changed(id: usize, new_job: JobType) {
    JOBS_QUEUE.lock().push_back(JobStep::JobChanged { id, new_job });
}

pub fn cancel_job(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::JobCancelled { id });
}

pub fn conclude_job(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::JobConcluded { id });
}

pub fn follow_job_path(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::FollowJobPath { id });
}

pub fn drop_item(id: usize, location: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::DropItem { id, location });
}

pub fn relinquish_claim(tool_id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::RelinquishClaim { tool_id });
}

pub fn equip_tool(id: usize, tool_id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::EquipItem { id, tool_id });
}

pub fn chop_tree(id: usize, tree_id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::TreeChop { id, tree_id });
}
