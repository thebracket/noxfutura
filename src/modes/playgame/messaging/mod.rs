mod queue_backend;
mod job_step;
mod apply;

use bracket_geometry::prelude::Point3;
use queue_backend::*;
use job_step::*;
use legion::*;

pub fn process_queues(ecs: &mut World, resources: &mut Resources) {
    apply::apply_jobs_queue(ecs, resources);
}

pub fn entity_moved(id: usize, end: &Point3) {
    JOBS_QUEUE.lock().push_back(JobStep::EntityMoved {
        id,
        end: end.clone(),
    });
}