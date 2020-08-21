use bracket_random::prelude::RandomNumberGenerator;
use nox_planet::Region;
use parking_lot::{Mutex, RwLock};

lazy_static! {
    pub static ref REGION: RwLock<Region> = RwLock::new(Region::initial());
}

lazy_static! {
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}
