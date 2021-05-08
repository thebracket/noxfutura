use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReactionJob {
    pub reaction_tag: String,
    pub workshop_id: usize,
    pub in_progress: Option<usize>,
}
