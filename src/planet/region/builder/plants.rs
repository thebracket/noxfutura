use crate::planet::{Region, TileType, REGION_HEIGHT, REGION_WIDTH};
use crate::utils::{ground_z, mapidx};
use bracket_random::prelude::RandomNumberGenerator;
use nox_raws::*;

pub fn grow_plants(region: &mut Region, mean_temperature: i8, rng: &mut RandomNumberGenerator) {
    let rlock = RAWS.read();

    for y in 0..REGION_HEIGHT {
        for x in 0..REGION_WIDTH {
            let z = ground_z(region, x, y);
            let idx = mapidx(x, y, z);
            if region.tile_types[idx] == TileType::Floor && region.water_level[idx] < 2 {
                let soil_quality = match rlock.materials.materials[region.material_idx[idx]].layer {
                    MaterialLayer::Sand => 1,
                    MaterialLayer::Soil { quality } => quality,
                    _ => 1,
                };
                //println!("Soil quality: {}", soil_quality);
                //println!("Topmost layer: {:#?}", rlock.materials.materials[region.material_idx[idx]].layer);

                let available_plants = rlock
                    .plants
                    .plants_by_hardiness_and_soil_quality(mean_temperature, soil_quality);
                if !available_plants.is_empty() {
                    if (rng.roll_dice(1, 15) as u8) <= soil_quality {
                        let chosen_plant = rng.random_slice_entry(&available_plants);
                        if let Some(plant_idx) = chosen_plant {
                            region.vegetation_type_id[idx] = Some(*plant_idx);
                        } else {
                            println!("Rejecting because no plant type was received");
                        }
                    } else {
                        //println!("Rejecting from dice roll");
                    }
                } else {
                    println!("Rejecting for lack of plant types");
                }
            }
        }
    }
}
