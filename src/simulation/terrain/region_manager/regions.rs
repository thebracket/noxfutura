use lazy_static::*;
use parking_lot::RwLock;
use std::collections::HashMap;
use crate::simulation::terrain::Region;

lazy_static! {
  pub(crate) static ref REGIONS: RwLock<Regions> = RwLock::new(Regions::new());
}

pub struct Regions {
    pub regions: HashMap<usize, Region>,
}

impl Regions {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
        }
    }
}