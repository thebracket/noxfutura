mod apply;
mod job_step;
mod queue_backend;

use bengine::{geometry::Point3, Palette};
use job_step::*;
use legion::*;
use queue_backend::*;

use nox_components::JobType;

pub fn process_queues(ecs: &mut World, resources: &mut Resources, palette: &Palette) {
    apply::apply_jobs_queue(ecs, resources, palette);
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

pub fn get_item(id: usize, item_id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::GetItem { id, item_id });
}

pub fn drop_item(id: usize, location: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::DropItem { id, location });
}

pub fn chop_tree(id: usize, tree_pos: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::TreeChop { id, tree_pos });
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

pub fn lights_changed() {
    JOBS_QUEUE.lock().push_back(JobStep::LightsChanged);
}

pub fn delete_item(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::DeleteItem { id });
}

pub fn delete_building(building_id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::DeleteBuilding { building_id });
}

pub fn finish_building(building_id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::FinishBuilding { building_id });
}

pub fn finish_construction(building_id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::FinishConstruction { building_id });
}

pub fn dig_at(id: usize, pos: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::DigAt { id, pos });
}

pub fn tile_dirty(pos: usize) {
    use nox_spatial::*;
    let mut jql = JOBS_QUEUE.lock();
    let (x, y, z) = idxmap(pos);
    for tz in -1i32..=1 {
        for ty in -1i32..=1 {
            for tx in -1i32..=1 {
                let idx = mapidx(
                    (x as i32 + tx) as usize,
                    (y as i32 + ty) as usize,
                    (z as i32 + tz) as usize,
                );
                jql.push_back(JobStep::TileDirty { pos: idx });
            }
        }
    }
    jql.push_back(JobStep::TileDirty { pos });
}

pub fn become_miner(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::BecomeMiner { id });
}

pub fn become_lumberjack(id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::BecomeLumberjack { id });
}

pub fn fire_miner(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::FireMiner { id });
}

pub fn fire_lumberjack(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::FireLumberjack { id });
}

pub fn spawn_item(position: &usize, tag: &String, qty: &i32, material: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::SpawnItem {
        pos: *position,
        tag: tag.clone(),
        qty: *qty as i32,
        material,
    });
}

pub fn haul_in_progress(id: usize, by: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::HaulInProgress { id, by })
}

pub fn reaction_in_progress(id: usize, by: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::ReactionInProgress { id, by })
}

pub fn construction_in_progress(id: usize, by: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::ConstructionInProgress {
            building_id: id,
            by,
        })
}

pub fn remove_haul_tag(id: usize) {
    JOBS_QUEUE.lock().push_back(JobStep::RemoveHaulTag { id })
}

pub fn update_blueprint(item_id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::UpdateBlueprint { item_id })
}

pub fn create_reaction_job(workshop_id: usize, reaction_tag: &str, components: &Vec<usize>) {
    JOBS_QUEUE.lock().push_back(JobStep::CreateReactionJob {
        workshop_id,
        components: components.clone(),
        reaction_tag: reaction_tag.to_string(),
    });
}

pub fn perform_reaction(reaction_id: usize) {
    JOBS_QUEUE
        .lock()
        .push_back(JobStep::PerformReaction { reaction_id });
}
