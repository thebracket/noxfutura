use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "38236c84-14c8-4dad-a378-db9855b18858"]
pub struct MyTurn{
    pub active: bool,
    pub shift: super::ScheduleTime
}
