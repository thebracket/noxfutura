use crate::planet::{ground_z, Region, TileType};
use crate::raws::*;
use crate::spatial::{mapidx, REGION_HEIGHT, REGION_WIDTH};
use bracket_random::prelude::RandomNumberGenerator;
use legion::*;

pub fn grow_plants(
    region: &mut Region,
    _ecs: &mut World,
    mean_temperature: i8,
    rng: &mut RandomNumberGenerator,
) {
    let rlock = RAWS.read();

    for y in 0..REGION_HEIGHT {
        for x in 0..REGION_WIDTH {
            let z = ground_z(region, x, y);
            let idx = mapidx(x, y, z);
            if region.is_floor(idx) && region.water_level[idx] < 2 {
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
                            region.tile_types[mapidx(x, y, z)] = TileType::Floor {
                                plant: Some(*plant_idx),
                            };
                        /*spawn_plant(
                            ecs,
                            &RAWS.read().plants.plants[*plant_idx].tag,
                            x,
                            y,
                            z,
                            region.world_idx,
                        );*/
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
