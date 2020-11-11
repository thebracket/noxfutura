use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Claimed{
    pub by : usize
}
