use serde::{Deserialize, Serialize};
use super::Planet;

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedGame {
    pub planet : Planet,
    pub current_region : crate::region::Region
}