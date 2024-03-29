use bengine::geometry::Point3;
use nox_components::JobType;

pub enum JobStep {
    EntityMoved {
        id: usize,
        end: Point3,
    },
    JobChanged {
        id: usize,
        new_job: JobType,
    },
    JobCancelled {
        id: usize,
    },
    JobConcluded {
        id: usize,
    },
    FollowJobPath {
        id: usize,
    },
    DropItem {
        id: usize,
        location: usize,
    },
    RelinquishClaim {
        tool_id: usize,
        tool_pos: usize,
    },
    GetItem {
        id: usize,
        item_id: usize,
    },
    EquipItem {
        id: usize,
        tool_id: usize,
    },
    TreeChop {
        id: usize,
        tree_pos: usize,
    },
    DeleteItem {
        id: usize,
    },
    FinishBuilding {
        building_id: usize,
    },
    FinishConstruction {
        building_id: usize,
    },
    DeleteBuilding {
        building_id: usize,
    },
    VoxMoved,
    ModelsMoved,
    LightsChanged,
    DigAt {
        id: usize,
        pos: usize,
    },
    TileDirty {
        pos: usize,
    },
    BecomeMiner {
        id: usize,
    },
    BecomeLumberjack {
        id: usize,
    },
    FireMiner {
        id: usize,
    },
    FireLumberjack {
        id: usize,
    },
    SpawnItem {
        pos: usize,
        tag: String,
        qty: i32,
        material: usize,
    },
    HaulInProgress {
        id: usize,
        by: usize,
    },
    ReactionInProgress {
        id: usize,
        by: usize,
    },
    RemoveHaulTag {
        id: usize,
    },
    UpdateBlueprint {
        item_id: usize,
    },
    CreateReactionJob {
        workshop_id: usize,
        components: Vec<usize>,
        reaction_tag: String,
    },
    PerformReaction {
        reaction_id: usize,
    },
    ConstructionInProgress {
        building_id: usize,
        by: usize,
    },
}
