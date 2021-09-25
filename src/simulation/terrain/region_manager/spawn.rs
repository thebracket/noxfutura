use crate::simulation::terrain::{PlanetLocation, Region, RegionRequirement};
use super::REGIONS;

pub fn spawn_playable_region(location: PlanetLocation) {
    let index = location.to_region_index();
    let mut region_lock = REGIONS.write();
    region_lock
        .regions
        .insert(index, Region::new(location, RegionRequirement::Required));
}
