use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "10d71e60-636a-456e-a15e-6fcaf1dbccb3"]
pub struct Tool {
    pub usage: ToolType
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "7cb11fdf-12ca-4857-935c-d1afe707d776"]
pub enum ToolType {
    Chopping, Digging, Farming
}