use crate::components::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Tool {
    pub usage: ToolType,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ToolType {
    Chopping,
    Digging,
    Farming,
}
