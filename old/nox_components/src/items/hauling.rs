use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct RequestHaul {
    pub destination: usize,
    pub in_progress: Option<usize>,
}
