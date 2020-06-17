use crate::components::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "17fd298a-7595-436a-a31e-23c9bdc31431"]
pub struct Tagline {
    pub name: String,
}
