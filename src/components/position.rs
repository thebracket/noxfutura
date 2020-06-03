use super::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "8c3902a4-c978-4874-b4d9-fbb781867a14"]
pub struct Position {
    pub x : i32,
    pub y : i32,
    pub z : i32
}