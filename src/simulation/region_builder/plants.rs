use crate::components::PlanetLocation;
use crate::raws::{MaterialLayer, RAWS};
use crate::simulation::region_builder::PLANET_STORE;
use crate::simulation::terrain::{
    get_material_idx, ground_z, is_tile_floor, submit_change_batch, ChangeRequest, MapChangeBatch,
};
use crate::simulation::{mapidx, REGION_HEIGHT, REGION_WIDTH};
use bracket_random::prelude::RandomNumberGenerator;

pub(crate) fn grow_plants(region_id: PlanetLocation) {
    let planet_lock = PLANET_STORE.read();
    let raw_lock = RAWS.read();
    let planet_idx = region_id.to_region_index();
    let mean_temperature =
        planet_lock.planet.as_ref().unwrap().landblocks[planet_idx].temperature_c as i8;

    let mut rng = RandomNumberGenerator::seeded(
        planet_lock.planet.as_ref().unwrap().noise_seed + region_id.to_region_index() as u64,
    );

    let mut changes = MapChangeBatch::new(region_id);
    for y in 0..REGION_HEIGHT {
        for x in 0..REGION_WIDTH {
            let z = ground_z(region_id, x, y);
            let tile_idx = mapidx(x, y, z);
            if is_tile_floor(region_id, tile_idx) {
                let material = get_material_idx(region_id, tile_idx);
                let soil_quality = match raw_lock.materials.materials[material].layer {
                    MaterialLayer::Soil { quality } => quality,
                    _ => 1,
                };
                let available_plants = raw_lock
                    .plants
                    .plants_by_hardiness_and_soil_quality(mean_temperature, soil_quality);
                if !available_plants.is_empty() {
                    if (rng.roll_dice(1, 10) as u8) <= soil_quality {
                        let chosen_plant = rng.random_slice_entry(&available_plants);
                        if let Some(plant_idx) = chosen_plant {
                            changes.enqueue_change(ChangeRequest::SpawnPlant {
                                idx: tile_idx,
                                plant_type: *plant_idx,
                            });
                        }
                    }
                }
            }
        }
    }
    submit_change_batch(changes);
}
