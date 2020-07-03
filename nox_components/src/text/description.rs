use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "45da198a-d0dd-4cc7-a679-06832ed9dfe1"]
pub struct Description {
    pub desc: String,
}
