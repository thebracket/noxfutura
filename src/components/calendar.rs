use super::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "a6464855-c959-4eb3-be05-dec8301b06b8"]
pub struct Calendar {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}
