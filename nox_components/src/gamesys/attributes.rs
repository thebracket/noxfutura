use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "bd5631c0-adcb-4353-987d-ca22bca42ae6"]
pub struct Attributes {
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
}
