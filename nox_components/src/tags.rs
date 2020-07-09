use super::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "abbd61e4-0e8c-41c9-813f-5e372f5cdca5"]
pub struct Cordex {}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "0e020e07-aa20-4de3-ad60-f1b2364abfc3"]
pub struct Building {}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "3e878aaa-b147-4d6f-8a03-ce0acdb26191"]
pub struct Item {}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "d21ed260-2438-417e-8701-6fb276c4ba09"]
pub struct Sentient {}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "64e4419f-908a-4de8-80eb-17008f572f7c"]
pub struct Vegetation {}