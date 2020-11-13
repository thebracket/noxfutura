use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Blueprint{
    pub required_items: Vec<usize>
}
