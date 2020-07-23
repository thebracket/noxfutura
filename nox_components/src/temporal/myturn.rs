use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "38236c84-14c8-4dad-a378-db9855b18858"]
pub struct MyTurn {
    pub active: bool,
    pub shift: crate::ScheduleTime,
    pub job: JobType,
    pub order: crate::WorkOrder,
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "31a41765-83fc-41db-8cbd-5a988020367d"]
pub enum JobType {
    None,
    FellTree {
        tree_id: usize,
        tree_pos: usize,
        tool_id: Option<usize>,
        step: LumberjackSteps,
    },
    ConstructBuilding {
        building_id: usize,
        building_pos: usize,
        step: BuildingSteps,
        components: Vec<(usize, usize, bool)>
    }
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "7733aa76-f2ff-4e91-835e-cf106def134f"]
pub enum LumberjackSteps {
    FindAxe,
    TravelToAxe { path: Vec<usize> },
    CollectAxe,
    FindTree,
    TravelToTree { path: Vec<usize> },
    ChopTree,
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "5d17017a-d6ed-47fb-b6ec-86950539261d"]
pub enum BuildingSteps {
    FindComponent,
    TravelToComponent { path: Vec<usize>, component_id: usize },
    CollectComponent { component_id: usize },
    FindBuilding { component_id: usize },
    TravelToTBuilding { path: Vec<usize>, component_id: usize },
    Construct,
}