use crate::planet::{Planet, WORLD_TILES_COUNT, WORLD_WIDTH, WORLD_HEIGHT, planet_idx, BlockType};
use super::{set_worldgen_status, PLANET_BUILD};

pub(crate) fn planet_determine_proportion(planet: &Planet, candidate: &mut i32, target: i32) -> u8 {
    let mut count = 0usize;
    while count < target as usize {
        count = planet.landblocks.iter().filter(|b| b.height < *candidate as u8).count();
        if count >= target as usize {
            return *candidate as u8;
        } else {
            *candidate += 1;
        }
    }
    0
}

pub(crate) fn planet_type_allocation() {
    set_worldgen_status("Dividing the waters from the earth");
    let mut candidate = 0;
    let mut planet = PLANET_BUILD.lock().planet.clone();
    let remaining_divisor = 10 - (planet.water_divisor + planet.plains_divisor);
    let n_cells = WORLD_TILES_COUNT as i32;
    let n_cells_water = n_cells / planet.water_divisor;
    let n_cells_plains = (n_cells / planet.plains_divisor) + n_cells_water;
    let n_cells_hills = (n_cells / remaining_divisor) + n_cells_plains;

    planet.water_height = planet_determine_proportion(&mut planet, &mut candidate, n_cells_water);
    planet.plains_height = planet_determine_proportion(&mut planet, &mut candidate, n_cells_plains);
    planet.hills_height = planet_determine_proportion(&mut planet, &mut candidate, n_cells_hills);

    for block in planet.landblocks.iter_mut() {
        if block.height <= planet.water_height {
            block.btype = BlockType::Water;
            block.rainfall = 10;
        }
        if block.height + block.variance/2 > planet.water_height {
            block.btype = BlockType::SaltMarsh;
        } else if block.height <= planet.plains_height {
            block.btype = BlockType::Plains;
            block.rainfall = 10;
            if block.height - block.variance/2 > planet.water_height {
                block.btype = BlockType::Marsh;
                block.rainfall = 20;
            }
        } else if block.height <= planet.hills_height {
            block.btype = BlockType::Hills;
            block.rainfall = 20;
            if block.variance < 2 {
                block.btype = BlockType::Highlands;
                block.rainfall = 10;
            }
        } else {
            block.btype = BlockType::Mountains;
            block.rainfall = 30;
            if block.variance < 3 {
                block.btype = BlockType::Plateau;
                block.rainfall = 10;
            }
        }
    }

    PLANET_BUILD.lock().planet.landblocks = planet.landblocks;
}

pub(crate) fn planet_coastlines() {
    set_worldgen_status("Crinkling the coastlines");
    let mut planet = PLANET_BUILD.lock().planet.clone();

    for y in 1..WORLD_HEIGHT as i32 -1 {
        for x in 1..WORLD_WIDTH as i32 - 1 {
            let base_idx = planet_idx(x, y);
            if planet.landblocks[base_idx].btype != BlockType::Water {
                if planet.landblocks[base_idx - 1].btype == BlockType::Water ||
                    planet.landblocks[base_idx + 1].btype == BlockType::Water ||
                    planet.landblocks[base_idx - WORLD_WIDTH as usize].btype == BlockType::Water ||
                    planet.landblocks[base_idx + WORLD_WIDTH as usize].btype == BlockType::Water 
                    {
                        planet.landblocks[base_idx].btype = BlockType::Coastal;
                        planet.landblocks[base_idx].rainfall = 20;
                    }
            }
        }
    }

    PLANET_BUILD.lock().planet.landblocks = planet.landblocks;
}

pub(crate) fn planet_rainfall() {
    set_worldgen_status("And then it rained a lot");
    let mut planet = PLANET_BUILD.lock().planet.clone();
    for y in 0..WORLD_HEIGHT as i32 {
        let mut rain_amount = 10;
        for x in 0..WORLD_WIDTH as i32 {
            let pidx = planet_idx(x, y);
            if planet.landblocks[pidx].btype == BlockType::Mountains {
                rain_amount -= 20;
            } else if planet.landblocks[pidx].btype == BlockType::Hills {
                rain_amount -= 10;
            } else if planet.landblocks[pidx].btype == BlockType::Coastal {
                rain_amount -= 5;
            } else {
                rain_amount += 1;
            }
            if rain_amount < 0 { rain_amount = 0; }
            if rain_amount > 20 { rain_amount = 20; }
            planet.landblocks[pidx].rainfall += rain_amount;
            if planet.landblocks[pidx].rainfall < 0 { planet.landblocks[pidx].rainfall = 0 }
            if planet.landblocks[pidx].rainfall > 100 { planet.landblocks[pidx].rainfall = 100 }
        }
    }
    PLANET_BUILD.lock().planet.landblocks = planet.landblocks;
}