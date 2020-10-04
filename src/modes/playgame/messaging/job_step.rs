use bracket_geometry::prelude::Point3;
use crate::components::JobType;


pub enum JobStep {
    EntityMoved { id: usize, end: Point3 },
    JobChanged { id: usize, new_job: JobType },
    JobCancelled { id: usize },
    JobConcluded { id: usize },
    FollowJobPath { id: usize },
    DropItem { id: usize, location: usize },
    RelinquishClaim { tool_id: usize, tool_pos: usize },
    EquipItem { id: usize, tool_id: usize },
    TreeChop { id: usize, tree_id: usize },
    DeleteItem { id: usize },
    FinishBuilding { building_id: usize },
    DeleteBuilding { building_id: usize },
    VoxMoved,
    ModelsMoved
}