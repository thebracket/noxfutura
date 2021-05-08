use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Construction {
    pub mode: usize,
    pub in_progress: Option<usize>,
}
