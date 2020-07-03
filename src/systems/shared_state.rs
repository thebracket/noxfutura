use crate::planet::Region;
use parking_lot::{RwLock, Mutex};
use bracket_random::prelude::RandomNumberGenerator;

lazy_static! {
    pub static ref REGION: RwLock<Region> = RwLock::new(Region::initial());
}

lazy_static! {
    pub static ref RNG : Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}