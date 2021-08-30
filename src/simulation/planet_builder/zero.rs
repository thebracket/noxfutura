use super::{planet_neighbors_four_way, BlockType, Direction, Landblock, Planet};
use crate::simulation::{planet_idx, WORLD_HEIGHT, WORLD_WIDTH};

pub fn zero_fill(planet: &mut Planet) {
    for y in 0..WORLD_HEIGHT {
        for x in 0..WORLD_WIDTH {
            planet.landblocks.push(Landblock {
                height: 0,
                variance: 0,
                btype: BlockType::None,
                temperature_c: 0.0,
                rainfall_mm: 0,
                biome_idx: usize::MAX,
                air_pressure_kpa: 0.0,
                prevailing_wind: Direction::None,
                neighbors: planet_neighbors_four_way(planet_idx(x, y)),
            });
        }
    }
}
