use super::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "0f8ff763-98ef-47e2-8b94-ba633ee78c5a"]
pub struct Name {
    pub name: String,
}
