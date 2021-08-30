use super::{BlockType, Planet, PlanetMesh, PLANET_GEN};
use crate::simulation::{planet_idx, WORLD_HEIGHT, WORLD_WIDTH};

pub fn planet_coastlines(planet: &mut Planet) {
    let mut n = 0;
    for y in 1..WORLD_HEIGHT - 1 {
        for x in 1..WORLD_WIDTH - 1 {
            let base_idx = planet_idx(x, y);
            if planet.landblocks[base_idx].btype != BlockType::Water {
                if planet.landblocks[base_idx - 1].btype == BlockType::Water
                    || planet.landblocks[base_idx + 1].btype == BlockType::Water
                    || planet.landblocks[base_idx - WORLD_WIDTH as usize].btype == BlockType::Water
                    || planet.landblocks[base_idx + WORLD_WIDTH as usize].btype == BlockType::Water
                {
                    planet.landblocks[base_idx].btype = BlockType::Coastal;
                    n += 1;
                    if n % 1000 == 0 {
                        let mut bumpy_planet = PlanetMesh::new();
                        bumpy_planet.with_category(&planet);
                        PLANET_GEN.write().globe_info = Some(bumpy_planet);
                    }
                }
            }
        }
    }
}
