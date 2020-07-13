mod backends;
mod renderflags;
use backends::*;
use nox_components::{Position, JobType};
mod jobstep;
use jobstep::JobStep;
pub use jobstep::apply_jobs_queue;
mod items;
use items::WorldChange;
pub use items::apply_world_queue;

pub use renderflags::get_render_flags;

pub fn reset() {
    RENDER_FLAGS.lock().reset();
}

pub fn vox_moved() {
    RENDER_FLAGS.lock().models_changed = true;
}

pub fn entity_moved(id: usize, end: &Position) {
    JOBS_QUEUE.lock().push(
        JobStep::EntityMoved{ id, end: end.clone() }
    );
}

pub fn job_changed(id: usize, new_job: JobType) {
    JOBS_QUEUE.lock().push(
        JobStep::JobChanged{ id, new_job }
    );
}

pub fn cancel_job(id: usize) {
    JOBS_QUEUE.lock().push(
        JobStep::JobCancelled{ id }
    );
}

pub fn conclude_job(id: usize) {
    JOBS_QUEUE.lock().push(
        JobStep::JobConcluded{ id }
    );
}

pub fn follow_job_path(id: usize) {
    JOBS_QUEUE.lock().push(
        JobStep::FollowJobPath{ id }
    );
}


pub fn equip_tool(id: usize, tool_id: usize) {
    WORLD_QUEUE.lock().push(
        WorldChange::EquipItem{ id, tool_id }
    );
}

pub fn chop_tree(id: usize, tree_id: usize) {
    WORLD_QUEUE.lock().push(
        WorldChange::TreeChop{ id, tree_id }
    );
}
