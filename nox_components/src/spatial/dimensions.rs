use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "d99a60aa-8264-4cef-b3c7-5d583d1edce1"]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}
