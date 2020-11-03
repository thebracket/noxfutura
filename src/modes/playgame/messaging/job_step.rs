use bengine::geometry::Point3;
use nox_components::JobType;

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
    ModelsMoved,
    LightsChanged,
    DigAt { id: usize, pos: usize },
    TileDirty { pos: usize },
    BecomeMiner { id: usize },
    BecomeLumberjack { id: usize },
    FireMiner { id: usize },
    FireLumberjack { id: usize },
}
