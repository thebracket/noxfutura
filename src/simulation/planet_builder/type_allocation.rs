use super::{BlockType, Planet, PlanetMesh, PLANET_GEN};
use crate::simulation::WORLD_TILES_COUNT;

pub fn planet_type_allocation(planet: &mut Planet) {
    const WATER_DIVISOR: usize = 3;
    const PLAINS_DIVISOR: usize = 3;
    const REMAINING_DIVISOR: usize = 10 - (WATER_DIVISOR + PLAINS_DIVISOR);
    let n_cells = WORLD_TILES_COUNT;
    let n_cells_water = n_cells / WATER_DIVISOR;
    let n_cells_plains = (n_cells / PLAINS_DIVISOR) + n_cells_water;
    let n_cells_hills = (n_cells / REMAINING_DIVISOR) + n_cells_plains;

    let mut candidate = 0;
    planet.water_height = planet_determine_proportion(planet, &mut candidate, n_cells_water as i32);
    planet.plains_height =
        planet_determine_proportion(planet, &mut candidate, n_cells_plains as i32);
    planet.hills_height = planet_determine_proportion(planet, &mut candidate, n_cells_hills as i32);

    for i in 0..planet.landblocks.len() {
        let mut block = &mut planet.landblocks[i];
        if block.height <= planet.water_height {
            block.btype = BlockType::Water;

            if block.height as u16 + block.variance as u16 / 2 > planet.water_height as u16 {
                block.btype = BlockType::SaltMarsh;
            }
        } else if block.height <= planet.plains_height {
            block.btype = BlockType::Plains;
            if block.height - block.variance < planet.water_height {
                block.btype = BlockType::Marsh;
            }
        } else if block.height <= planet.hills_height {
            block.btype = BlockType::Hills;
            if block.variance < 2 {
                block.btype = BlockType::Highlands;
            }
        } else {
            block.btype = BlockType::Mountains;
            if block.variance < 3 {
                block.btype = BlockType::Plateau;
            }
        }

        if i % 500 == 0 {
            let mut bumpy_planet = PlanetMesh::new();
            bumpy_planet.with_category(&planet);
            PLANET_GEN.write().globe_info = Some(bumpy_planet);
        }
    }
}

fn planet_determine_proportion(planet: &Planet, candidate: &mut i32, target: i32) -> u8 {
    let mut count = 0usize;
    while count < target as usize {
        count = planet
            .landblocks
            .iter()
            .filter(|b| b.height <= *candidate as u8)
            .count();
        if count >= target as usize {
            return *candidate as u8;
        } else {
            *candidate += 1;
        }
    }
    0
}
