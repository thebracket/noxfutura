use crate::planet::Region;
use parking_lot::RwLock;

lazy_static! {
    pub static ref REGION: RwLock<Region> = RwLock::new(Region::initial());
}
